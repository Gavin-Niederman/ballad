use std::cell::LazyCell;

use gtk::glib::{self, Object};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::gio::{Cancellable, DBusProxy, DBusProxyFlags};
    use gtk::glib::clone;
    use gtk::{gio, glib};
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::BatteryService)]
    pub struct BatteryService {
        #[property(get)]
        available: Cell<bool>,
        #[property(get)]
        percentage: Cell<f64>,
        #[property(get)]
        charging: Cell<bool>,
        #[property(get)]
        charged: Cell<bool>,
        #[property(get)]
        time_remaining: Cell<f64>,
        #[property(get)]
        energy: Cell<f64>,
        #[property(get)]
        energy_full: Cell<f64>,
        #[property(get)]
        energy_rate: Cell<f64>,

        proxy: RefCell<Option<DBusProxy>>,
    }

    impl BatteryService {
        fn update(&self) {
            let proxy = self.proxy.borrow();
            let proxy = proxy.as_ref().unwrap();

            let available = proxy.cached_property("IsPresent").unwrap().get().unwrap();
            self.available.replace(available);
            self.obj().notify_available();
            if !available {
                return;
            }

            let percentage: f64 = proxy.cached_property("Percentage").unwrap().get().unwrap();
            self.percentage.replace(percentage);
            self.obj().notify_percentage();

            let state: u32 = proxy.cached_property("State").unwrap().get().unwrap();
            self.charging.replace(state == 1);
            self.charged.replace(state == 4);
            self.obj().notify_charging();
            self.obj().notify_charged();

            let time_to_full = proxy.cached_property("TimeToFull");
            let time_to_empty = proxy.cached_property("TimeToEmpty");
            let time_remaining = time_to_full.or(time_to_empty).unwrap().get().unwrap();
            self.time_remaining.replace(time_remaining);
            self.obj().notify_time_remaining();

            let energy: f64 = proxy.cached_property("Energy").unwrap().get().unwrap();
            self.energy.replace(energy);
            self.obj().notify_energy();

            let energy_full: f64 = proxy.cached_property("EnergyFull").unwrap().get().unwrap();
            self.energy_full.replace(energy_full);
            self.obj().notify_energy_full();

            let energy_rate: f64 = proxy.cached_property("EnergyRate").unwrap().get().unwrap();
            self.energy_rate.replace(energy_rate);
            self.obj().notify_energy_rate();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BatteryService {
        const NAME: &'static str = "BalladServicesBatteryService";
        type Type = super::BatteryService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for BatteryService {
        fn constructed(&self) {
            self.parent_constructed();

            let proxy = DBusProxy::for_bus_sync(
                gio::BusType::System,
                DBusProxyFlags::empty(),
                None,
                "org.freedesktop.UPower.Device",
                "/org/freedesktop/UPower/devices/DisplayDevice",
                "org.freedesktop.UPower",
                Cancellable::NONE,
            )
            .unwrap();

            proxy.connect_notify_local(
                None,
                clone!(
                    #[weak(rename_to = this)]
                    self,
                    move |_, _| {
                        this.update();
                        println!("Battery updated");
                    }
                ),
            );

            self.proxy.replace(Some(proxy));
        }
    }
}

glib::wrapper! {
    pub struct BatteryService(ObjectSubclass<imp::BatteryService>);
}

impl BatteryService {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
impl Default for BatteryService {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static BATTERY_SERVICE: LazyCell<BatteryService> = LazyCell::new(BatteryService::new);
}
