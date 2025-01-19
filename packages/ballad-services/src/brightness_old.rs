use std::cell::LazyCell;

use gtk::glib;

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::gio::{Cancellable, FileMonitorFlags};
    use gtk::glib::{self, Properties, clone};
    use gtk::{prelude::*, subclass::prelude::*};

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::BrightnessService)]
    pub struct BrightnessService {
        #[property(get, set = |this: &Self, b| smol::block_on(this.set_brightness(b)))]
        brightness: Cell<f64>,
        #[property(get)]
        available: Cell<bool>,

        device: RefCell<Option<String>>,
        watcher: RefCell<Option<gtk::gio::FileMonitor>>,
    }
    impl BrightnessService {
        async fn update(&self) {
            if !self.available.get() {
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

            self.brightness
                .set(current_brightness as f64 / max_brightness as f64);
            self.obj().notify_brightness();
        }
        async fn set_brightness(&self, brightness: f64) {
            if brightness != self.brightness.get() {
                self.brightness.set(brightness);
                self.obj().notify_brightness();

                let percent = format!("{}%", brightness * 100.0);
                let _ = smol::process::Command::new("brightnessctl")
                    .arg("s")
                    .arg(percent)
                    .output()
                    .await;
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BrightnessService {
        const NAME: &'static str = "BalladServicesBrightnessService";
        type Type = super::BrightnessService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BrightnessService {
        fn constructed(&self) {
            self.parent_constructed();

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
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
                        println!(
                            "No backlight devices found. Brightness service will not function!"
                        );
                        this.available.set(false);
                        this.obj().notify_available();
                        return;
                    };
                    this.device.replace(Some(device.to_string()));

                    this.available.set(true);
                    this.obj().notify_available();

                    this.update().await;

                    let f = gtk::gio::File::for_path(format!(
                        "/sys/class/backlight/{device}/brightness"
                    ));
                    let watcher = f
                        .monitor(FileMonitorFlags::NONE, Cancellable::NONE)
                        .unwrap();
                    this.watcher.replace(Some(watcher.clone()));

                    watcher.connect_changed(move |_, _, _, _| {
                        smol::block_on(this.update());
                    });
                }
            ));
        }
    }
}

glib::wrapper! {
    pub struct BrightnessService(ObjectSubclass<imp::BrightnessService>);
}
impl BrightnessService {
    pub fn new() -> Self {
        glib::Object::builder().build()
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
