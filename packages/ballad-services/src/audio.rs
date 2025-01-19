use std::{cell::LazyCell, time::Duration};

use ballad_macro::Reactive;
use futures::{FutureExt, select_biased};
use libpulse_binding::{
    context::subscribe::{Facility, InterestMaskSet, Operation},
    mainloop::standard::IterateResult,
    volume::Volume,
};
use pulsectl::{
    Handler,
    controllers::{DeviceControl, SinkController},
};
use smol::{
    Timer,
    channel::{Receiver, Sender},
};

use crate::{reactive::Reactive, reactive_wrapper};

#[derive(Debug, Clone, Copy, PartialEq)]
enum AudioChange {
    Volume(f64),
    Muted(bool),
    All(f64, bool),
}

fn get_volume(controller: &mut SinkController) -> (f64, bool) {
    let default_dev = controller.get_default_device().unwrap();
    let full_vol = default_dev.base_volume.0;
    let vol = default_dev.volume.get()[0];
    (vol.0 as f64 / full_vol as f64, default_dev.mute)
}

fn start_pulse_daemon() -> (Sender<AudioChange>, Receiver<AudioChange>) {
    let (command_sender, command_receiver) = smol::channel::unbounded();
    let (event_sender, event_receiver) = smol::channel::bounded(5);

    std::thread::spawn(move || {
        let mut handler = Handler::connect("Ballad Shell").unwrap();
        let subscribe_op = handler
            .context
            .borrow_mut()
            .subscribe(InterestMaskSet::all(), |_| {});
        handler.wait_for_operation(subscribe_op).unwrap();

        let (notifier, receiver) = smol::channel::unbounded();
        handler
            .context
            .borrow_mut()
            .set_subscribe_callback(Some(Box::new(move |facility, operation, _index| {
                if facility.is_some_and(|facility| {
                    facility == Facility::Card
                        || facility == Facility::Sink
                        || facility == Facility::SourceOutput
                }) && operation.is_some_and(|op| op == Operation::Changed)
                {
                    _ = notifier.send_blocking(());
                }
            })));

        let mut sink_controller = SinkController { handler };

        smol::block_on(async {
            let mut last_volume = None;
            let mut last_muted = None;

            let (vol, muted) = get_volume(&mut sink_controller);
            _ = event_sender.send(AudioChange::All(vol, muted)).await;

            loop {
                match sink_controller.handler.mainloop.borrow_mut().iterate(false) {
                    IterateResult::Success(_) => {}
                    IterateResult::Quit(_) | IterateResult::Err(_) => {
                        break;
                    }
                }

                select_biased! {
                    res = receiver.recv().fuse() => {
                        if res.is_err() {
                            break;
                        }

                        let (vol, muted) = get_volume(&mut sink_controller);

                        if last_volume != Some(vol) {
                            last_volume = Some(vol);
                            _ = event_sender.send(AudioChange::Volume(vol)).await;
                        }
                        if last_muted != Some(muted) {
                            last_muted = Some(muted);
                            _ = event_sender.send(AudioChange::Muted(muted)).await;
                        }
                    }
                    res = command_receiver.recv().fuse() => {
                        if let Ok(change) = res {
                            let mut default_dev = sink_controller.get_default_device().unwrap();

                            match change {
                                AudioChange::Volume(volume) => {
                                    let num_channels = default_dev.channel_map.len();
                                    let full_vol = default_dev.base_volume.0;
                                    let v = (volume * full_vol as f64) as u32;
                                    sink_controller.set_device_volume_by_index(default_dev.index, default_dev.volume.set(num_channels, Volume(v)));

                                    last_volume = Some(volume);
                                }
                                AudioChange::Muted(muted) => {
                                    sink_controller.set_device_mute_by_index(default_dev.index, muted);

                                    last_muted = Some(muted);
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            break;
                        }
                    }
                    _ = Timer::after(Duration::from_millis(10)).fuse() => {}
                }
            }
        })
    });

    (command_sender, event_receiver)
}

#[derive(Debug, Clone, Reactive)]
#[wrapper_type(AudioService)]
struct AudioServiceInner {
    #[property(get)]
    volume: f64,
    #[property(get)]
    muted: bool,

    command_sender: Sender<AudioChange>,
}

reactive_wrapper!(pub AudioService<AudioServiceInner, Weak = WeakAudioServiceInner>);

impl AudioService {
    pub fn new() -> Self {
        let (command_sender, event_receiver) = start_pulse_daemon();

        let this = Self {
            inner: Reactive::new(AudioServiceInner {
                volume: 0.0,
                muted: false,
                command_sender,
            }),
        };

        if let Ok(AudioChange::All(vol, muted)) = event_receiver.recv_blocking() {
            this.inner.apply(|inner| {
                inner.volume = vol;
                inner.muted = muted;
            });
        }

        let this2 = this.clone();
        gtk::glib::spawn_future_local(async move {
            while let Ok(received) = event_receiver.recv().await {
                match received {
                    AudioChange::Volume(volume) => this2.inner.apply(|inner| inner.volume = volume),
                    AudioChange::Muted(muted) => this2.inner.apply(|inner| inner.muted = muted),
                    AudioChange::All(volume, muted) => {
                        dbg!(volume, muted);
                        this2.inner.apply(|inner| {
                            inner.volume = volume;
                            inner.muted = muted;
                        });
                    }
                }
            }
        });

        this
    }

    pub async fn set_volume(&self, volume: f64) {
        let inner = self.inner.get().await;
        _ = inner.command_sender.send(AudioChange::Volume(volume)).await;
        self.inner.apply(|inner| inner.volume = volume);
    }
    pub fn set_volume_blocking(&self, volume: f64) {
        smol::block_on(self.set_volume(volume));
    }
    pub async fn set_muted(&self, muted: bool) {
        _ = self
            .inner
            .get()
            .await
            .command_sender
            .send(AudioChange::Muted(muted))
            .await;
        self.inner.apply(|inner| inner.muted = muted);
    }
    pub fn set_muted_blocking(&self, muted: bool) {
        smol::block_on(self.set_muted(muted));
    }
}

impl Default for AudioService {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static AUDIO_SERVICE: LazyCell<AudioService> = LazyCell::new(AudioService::new);
}
