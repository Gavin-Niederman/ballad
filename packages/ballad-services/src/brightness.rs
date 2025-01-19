use std::cell::LazyCell;

use ballad_macro::Reactive;

use gtk::{
    gio::{Cancellable, FileMonitorFlags},
    prelude::*,
};

use crate::{reactive::Reactive, reactive_wrapper};

#[derive(Debug, Clone, Reactive, Default)]
#[wrapper_type(BrightnessService)]
struct BrightnessServiceInner {
    #[property(get)]
    brightness: f64,
    #[property(get)]
    available: bool,

    device: Option<String>,
    watcher: Option<gtk::gio::FileMonitor>,
}

impl BrightnessServiceInner {
    async fn update(&mut self) {
        if !self.available {
            return;
        }

        let (Ok(max_brightness), Ok(current_brightness)) = futures::join!(
            smol::process::Command::new("brightnessctl")
                .arg("m")
                .output(),
            smol::process::Command::new("brightnessctl")
                .arg("g")
                .output(),
        ) else {
            return;
        };

        let max_brightness = String::from_utf8(max_brightness.stdout)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        let current_brightness = String::from_utf8(current_brightness.stdout)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        self.brightness = current_brightness as f64 / max_brightness as f64;
    }
    async fn set_brightness(&mut self, brightness: f64) {
        if brightness != self.brightness {
            self.brightness = brightness;

            let percent = format!("{}%", brightness * 100.0);
            let _ = smol::process::Command::new("brightnessctl")
                .arg("s")
                .arg(percent)
                .output()
                .await;
        }
    }
}

reactive_wrapper!(pub BrightnessService<BrightnessServiceInner, Weak = WeakBrightnessService>);

impl BrightnessService {
    pub fn new() -> Self {
        let mut this = Self {
            inner: Reactive::new(BrightnessServiceInner::default()),
        };
        smol::block_on(this.setup());
        this
    }

    async fn setup(&mut self) {
        let mut this = self.inner.get().await;

        let Some(device) = smol::process::Command::new("ls")
            .args(["-w1", "/sys/class/backlight/"])
            .output()
            .await
            .ok()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .unwrap()
                    .lines()
                    .next()
                    .map(|s| s.to_string())
            })
        else {
            println!("No backlight devices found. Brightness service will not function!");
            this.available = false;
            return;
        };
        this.device.replace(device.to_string());

        this.available = true;

        this.update().await;

        let f = gtk::gio::File::for_path(format!("/sys/class/backlight/{device}/brightness"));
        let watcher = f
            .monitor(FileMonitorFlags::NONE, Cancellable::NONE)
            .unwrap();
        this.watcher.replace(watcher.clone());

        let service = self.clone();
        watcher.connect_changed(move |_, _, _, _| {
            service.inner.apply(|inner| {
                smol::block_on(inner.update());
            });
        });
    }

    pub fn set_brightness(&self, brightness: f64) {
        self.inner
            .apply(|inner| smol::block_on(inner.set_brightness(brightness)));
    }
}

impl Default for BrightnessService {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static BRIGHTNESS_SERVICE: LazyCell<BrightnessService> = LazyCell::new(BrightnessService::new);
}
