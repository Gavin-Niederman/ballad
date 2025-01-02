use std::cell::LazyCell;

use gtk::glib;

mod audio_imp {
    use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};

    pub struct SystemSound {
        mixer: Mixer,
        master_audio: SelemId,
    }
    impl SystemSound {
        pub fn new() -> Option<Self> {
            let Ok(mixer) = Mixer::new("default", false) else {
                println!(
                    "Failed to open default mixer. Ballad Audio services do not work in containers!"
                );
                return None;
            };
            let master_audio = mixer
                .iter()
                .filter_map(Selem::new)
                .find(|elem| elem.can_playback() && elem.has_playback_volume())
                .map(|selem| selem.get_id())?;

            Some(Self {
                mixer,
                master_audio,
            })
        }

        fn get_master_selem(&self) -> Selem<'_> {
            self.mixer.find_selem(&self.master_audio).unwrap()
        }

        pub fn get_volume(&self) -> f64 {
            let selem = self.get_master_selem();

            let (min, max) = selem.get_playback_volume_range();
            let volume = selem.get_playback_volume(SelemChannelId::mono()).unwrap();

            (volume - min) as f64 / (max - min) as f64
        }
        pub fn set_volume(&self, volume: f64) {
            let selem = self.get_master_selem();

            let (min, max) = selem.get_playback_volume_range();
            let volume = (volume * (max - min) as f64 + min as f64) as i64;

            selem.set_playback_volume_all(volume).unwrap();
        }

        pub fn get_muted(&self) -> bool {
            self.get_master_selem()
                .get_playback_switch(SelemChannelId::mono())
                .unwrap()
                == 0
        }
        pub fn set_muted(&self, muted: bool) {
            self.get_master_selem()
                .set_playback_switch_all((!muted).into())
                .unwrap();
        }

        pub fn tick(&mut self) {
            _ = self.mixer.handle_events();
        }
    }
}

mod gobject_imp {
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    use gtk::gio;
    use gtk::glib::subclass::Signal;
    use gtk::glib::{self, clone};
    use gtk::{prelude::*, subclass::prelude::*};
    use smol::channel::TryRecvError;

    use super::audio_imp;

    enum AudioChange {
        Volume(f64),
        Muted(bool),
        All(f64, bool),
    }

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::AudioService)]
    pub struct AudioService {
        #[property(get, set = AudioService::set_volume)]
        volume: Cell<f64>,
        #[property(get, set = AudioService::set_muted)]
        muted: Cell<bool>,

        command_sender: RefCell<Option<smol::channel::Sender<AudioChange>>>,
    }

    impl AudioService {
        fn set_volume(&self, volume: f64) {
            _ = self
                .command_sender
                .borrow()
                .as_ref()
                .unwrap()
                .try_send(AudioChange::Volume(volume));
            self.volume.set(volume);
        }
        fn set_muted(&self, muted: bool) {
            _ = self
                .command_sender
                .borrow()
                .as_ref()
                .unwrap()
                .try_send(AudioChange::Muted(muted));
            self.muted.set(muted);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AudioService {
        const NAME: &'static str = "BalladServicesAudioService";
        type Type = super::AudioService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AudioService {
        fn constructed(&self) {
            self.parent_constructed();

            let (command_sender, command_receiver) = smol::channel::bounded(1);
            let (update_sender, update_receiver) = smol::channel::bounded(1);

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    while let Ok(received) = update_receiver.recv().await {
                        match received {
                            AudioChange::Volume(volume) => {
                                this.volume.set(volume);
                                this.obj().notify_volume();
                            }
                            AudioChange::Muted(muted) => {
                                this.muted.set(muted);
                                this.obj().notify_muted();
                            }
                            AudioChange::All(volume, muted) => {
                                this.volume.set(volume);
                                this.muted.set(muted);
                                this.obj().notify_volume();
                                this.obj().notify_muted();
                            }
                        }
                        this.obj().emit_by_name::<()>("audio-changed", &[]);
                    }
                }
            ));

            gio::spawn_blocking(move || {
                let mut system_sound = audio_imp::SystemSound::new().unwrap();

                let mut last_volume = system_sound.get_volume();
                let mut last_muted = system_sound.get_muted();

                update_sender
                    .try_send(AudioChange::All(last_volume, last_muted))
                    .unwrap();

                loop {
                    match command_receiver.try_recv() {
                        Err(TryRecvError::Closed) => break,
                        Ok(AudioChange::Muted(muted)) => {
                            system_sound.set_muted(muted);
                        }
                        Ok(AudioChange::Volume(volume)) => {
                            system_sound.set_volume(volume);
                        }
                        Ok(AudioChange::All(volume, muted)) => {
                            system_sound.set_volume(volume);
                            system_sound.set_muted(muted);
                        }
                        _ => {}
                    }

                    let volume = system_sound.get_volume();
                    let muted = system_sound.get_muted();

                    if volume != last_volume {
                        _ = update_sender.try_send(AudioChange::Volume(volume));
                        last_volume = volume;
                    }
                    if muted != last_muted {
                        _ = update_sender.try_send(AudioChange::Muted(muted));
                        last_muted = muted;
                    }

                    system_sound.tick();
                }
            });

            self.command_sender.replace(Some(command_sender));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("audio-changed").build()])
        }
    }
}

glib::wrapper! {
    pub struct AudioService(ObjectSubclass<gobject_imp::AudioService>);
}
impl AudioService {
    pub fn new() -> Self {
        glib::Object::builder().build()
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
