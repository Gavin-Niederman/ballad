use std::cell::LazyCell;

use gtk::glib::{self, Object};

mod imp {
    use std::cell::Cell;
    use std::sync::OnceLock;

    use futures::join;
    use gtk::glib::clone;
    use gtk::glib::subclass::Signal;

    use gtk::glib;
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};
    use smol::lock::RwLock;
    use smol::stream::StreamExt;
    use zbus::proxy;

    use crate::DBUS_SYSTEM_CONNECTION;

    #[proxy(
        interface = "org.freedesktop.UPower.Device",
        default_service = "org.freedesktop.UPower",
        default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
    )]
    trait UPowerDevice {
        /// GetHistory method
        fn get_history(
            &self,
            type_: &str,
            timespan: u32,
            resolution: u32,
        ) -> zbus::Result<Vec<(u32, f64, u32)>>;

        /// GetStatistics method
        fn get_statistics(&self, type_: &str) -> zbus::Result<Vec<(f64, f64)>>;

        /// Refresh method
        fn refresh(&self) -> zbus::Result<()>;

        /// BatteryLevel property
        #[zbus(property)]
        fn battery_level(&self) -> zbus::Result<u32>;

        /// Capacity property
        #[zbus(property)]
        fn capacity(&self) -> zbus::Result<f64>;

        /// ChargeCycles property
        #[zbus(property)]
        fn charge_cycles(&self) -> zbus::Result<i32>;

        /// Energy property
        #[zbus(property)]
        fn energy(&self) -> zbus::Result<f64>;

        /// EnergyEmpty property
        #[zbus(property)]
        fn energy_empty(&self) -> zbus::Result<f64>;

        /// EnergyFull property
        #[zbus(property)]
        fn energy_full(&self) -> zbus::Result<f64>;

        /// EnergyFullDesign property
        #[zbus(property)]
        fn energy_full_design(&self) -> zbus::Result<f64>;

        /// EnergyRate property
        #[zbus(property)]
        fn energy_rate(&self) -> zbus::Result<f64>;

        /// HasHistory property
        #[zbus(property)]
        fn has_history(&self) -> zbus::Result<bool>;

        /// HasStatistics property
        #[zbus(property)]
        fn has_statistics(&self) -> zbus::Result<bool>;

        /// IconName property
        #[zbus(property)]
        fn icon_name(&self) -> zbus::Result<String>;

        /// IsPresent property
        #[zbus(property)]
        fn is_present(&self) -> zbus::Result<bool>;

        /// IsRechargeable property
        #[zbus(property)]
        fn is_rechargeable(&self) -> zbus::Result<bool>;

        /// Luminosity property
        #[zbus(property)]
        fn luminosity(&self) -> zbus::Result<f64>;

        /// Model property
        #[zbus(property)]
        fn model(&self) -> zbus::Result<String>;

        /// NativePath property
        #[zbus(property)]
        fn native_path(&self) -> zbus::Result<String>;

        /// Online property
        #[zbus(property)]
        fn online(&self) -> zbus::Result<bool>;

        /// Percentage property
        #[zbus(property)]
        fn percentage(&self) -> zbus::Result<f64>;

        /// PowerSupply property
        #[zbus(property)]
        fn power_supply(&self) -> zbus::Result<bool>;

        /// Serial property
        #[zbus(property)]
        fn serial(&self) -> zbus::Result<String>;

        /// State property
        #[zbus(property)]
        fn state(&self) -> zbus::Result<u32>;

        /// Technology property
        #[zbus(property)]
        fn technology(&self) -> zbus::Result<u32>;

        /// Temperature property
        #[zbus(property)]
        fn temperature(&self) -> zbus::Result<f64>;

        /// TimeToEmpty property
        #[zbus(property)]
        fn time_to_empty(&self) -> zbus::Result<i64>;

        /// TimeToFull property
        #[zbus(property)]
        fn time_to_full(&self) -> zbus::Result<i64>;

        /// Type property
        #[zbus(property)]
        fn type_(&self) -> zbus::Result<u32>;

        /// UpdateTime property
        #[zbus(property)]
        fn update_time(&self) -> zbus::Result<u64>;

        /// Vendor property
        #[zbus(property)]
        fn vendor(&self) -> zbus::Result<String>;

        /// Voltage property
        #[zbus(property)]
        fn voltage(&self) -> zbus::Result<f64>;

        /// WarningLevel property
        #[zbus(property)]
        fn warning_level(&self) -> zbus::Result<u32>;
    }

    #[derive(Properties, Default, Debug)]
    #[properties(wrapper_type = super::UPowerService)]
    pub struct UPowerService {
        #[property(get)]
        available: Cell<bool>,
        #[property(get)]
        percentage: Cell<f64>,
        #[property(get)]
        charging: Cell<bool>,
        #[property(get)]
        charged: Cell<bool>,
        #[property(get)]
        time_remaining: Cell<u64>,
        #[property(get)]
        energy: Cell<f64>,
        #[property(get)]
        energy_full: Cell<f64>,
        #[property(get)]
        energy_rate: Cell<f64>,

        proxy: RwLock<Option<UPowerDeviceProxy<'static>>>,
    }

    impl UPowerService {
        async fn update(&self) {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();

            let available = proxy.is_present().await.unwrap_or_default();
            self.available.replace(available);
            self.obj().notify_available();
            if !available {
                self.obj().emit_by_name::<()>("battery-changed", &[]);
                return;
            }

            let (percentage, state, time_to_full, time_to_empty, energy, energy_full, energy_rate) = join!(
                proxy.percentage(),
                proxy.state(),
                proxy.time_to_full(),
                proxy.time_to_empty(),
                proxy.energy(),
                proxy.energy_full(),
                proxy.energy_rate(),
            );

            let percentage = percentage.unwrap_or_default();
            self.percentage.replace(percentage);
            self.obj().notify_percentage();

            let state = state.unwrap_or_default();
            self.charging.replace(state == 1);
            self.charged.replace(state == 4 || percentage == 100.0);
            self.obj().notify_charging();
            self.obj().notify_charged();

            let time_to_full = time_to_full.ok();
            let time_to_empty = time_to_empty.ok();
            let time_remaining = time_to_full
                .or(time_to_empty)
                .and_then(|val| val.try_into().ok())
                .unwrap_or_default();
            self.time_remaining.replace(time_remaining);
            self.obj().notify_time_remaining();

            let energy = energy.unwrap_or_default();
            self.energy.replace(energy);
            self.obj().notify_energy();

            let energy_full = energy_full.unwrap_or_default();
            self.energy_full.replace(energy_full);
            self.obj().notify_energy_full();

            let energy_rate = energy_rate.unwrap_or_default();
            self.energy_rate.replace(energy_rate);
            self.obj().notify_energy_rate();

            self.obj().emit_by_name::<()>("battery-changed", &[]);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UPowerService {
        const NAME: &'static str = "BalladServicesBatteryService";
        type Type = super::UPowerService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for UPowerService {
        fn constructed(&self) {
            self.parent_constructed();

            gtk::glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let Ok(proxy) = UPowerDeviceProxy::new(&DBUS_SYSTEM_CONNECTION).await else {
                        println!(
                            "Failed to create UPowerDeviceProxy. Battery and brightness services will not function!"
                        );
                        return;
                    };

                    this.proxy.write().await.replace(proxy.clone());
                    this.update().await;

                    let mut update_stream = proxy.inner().receive_all_signals().await.unwrap();
                    while update_stream.next().await.is_some() {
                        this.update().await;
                    }
                }
            ));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("battery-changed").build()])
        }
    }
}

glib::wrapper! {
    pub struct UPowerService(ObjectSubclass<imp::UPowerService>);
}

impl UPowerService {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
impl Default for UPowerService {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static BATTERY_SERVICE: LazyCell<UPowerService> = LazyCell::new(UPowerService::new);
}
