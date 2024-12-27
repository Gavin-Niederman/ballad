use std::cell::LazyCell;

use gtk::glib::{self, Object};

mod imp {
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    use gtk::gio::{Cancellable, DBusProxy, DBusProxyFlags};
    use gtk::glib::subclass::Signal;
    use gtk::glib::{Value, Variant, closure_local};
    use gtk::{gio, glib};
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    #[derive(Properties, Default, Debug)]
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
            fn get_property<T: FromVariant>(proxy: &DBusProxy, property: &str) -> Option<T> {
                proxy
                    .cached_property(property)
                    .and_then(|property| property.get())
            }

            let proxy = self.proxy.borrow();
            let proxy = proxy.as_ref().unwrap();

            let available = get_property::<bool>(proxy, "IsPresent").unwrap_or_default();
            self.available.replace(available);
            self.obj().notify_available();
            if !available {
                self.obj().emit_by_name::<()>("battery-changed", &[]);
                return;
            }

            let percentage = get_property(proxy, "Percentage").unwrap_or_default();
            self.percentage.replace(percentage);
            self.obj().notify_percentage();

            let state: u32 = get_property(proxy, "State").unwrap_or_default();
            self.charging.replace(state == 1);
            self.charged.replace(state == 4 || percentage == 100.0);
            self.obj().notify_charging();
            self.obj().notify_charged();

            let time_to_full = get_property(proxy, "TimeToFull");
            let time_to_empty = get_property(proxy, "TimeToEmpty");
            let time_remaining = time_to_full.or(time_to_empty).unwrap_or_default();
            self.time_remaining.replace(time_remaining);
            self.obj().notify_time_remaining();

            let energy = get_property(proxy, "Energy").unwrap_or_default();
            self.energy.replace(energy);
            self.obj().notify_energy();

            let energy_full = get_property(proxy, "EnergyFull").unwrap_or_default();
            self.energy_full.replace(energy_full);
            self.obj().notify_energy_full();

            let energy_rate = get_property(proxy, "EnergyRate").unwrap_or_default();
            self.energy_rate.replace(energy_rate);
            self.obj().notify_energy_rate();

            self.obj().emit_by_name::<()>("battery-changed", &[]);
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
                "org.freedesktop.UPower",
                "/org/freedesktop/UPower/devices/DisplayDevice",
                "org.freedesktop.UPower.Device",
                Cancellable::NONE,
            )
            .unwrap();

            proxy.connect_closure(
                "g-properties-changed",
                false,
                closure_local!(
                    #[weak(rename_to = this)]
                    self,
                    move |_: DBusProxy, _: Variant, _: Value| {
                        this.update();
                    }
                ),
            );

            self.proxy.replace(Some(proxy));

            self.update();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("battery-changed").build()])
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
