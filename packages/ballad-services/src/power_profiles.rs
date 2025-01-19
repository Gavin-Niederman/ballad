use ballad_macro::Reactive;
use futures::join;
use zbus::proxy;

use crate::{DBUS_SYSTEM_CONNECTION, reactive_wrapper};

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
trait PowerProfiles {
    /// HoldProfile method
    fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> zbus::Result<u32>;

    /// ReleaseProfile method
    fn release_profile(&self, cookie: u32) -> zbus::Result<()>;

    /// ProfileReleased signal
    #[zbus(signal)]
    fn profile_released(&self, cookie: u32) -> zbus::Result<()>;

    /// Actions property
    #[zbus(property)]
    fn actions(&self) -> zbus::Result<Vec<String>>;

    /// ActiveProfile property
    #[zbus(property)]
    fn active_profile(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_active_profile(&self, value: &str) -> zbus::Result<()>;

    /// ActiveProfileHolds property
    #[zbus(property)]
    fn active_profile_holds(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// PerformanceDegraded property
    #[zbus(property)]
    fn performance_degraded(&self) -> zbus::Result<String>;

    /// PerformanceInhibited property
    #[zbus(property)]
    fn performance_inhibited(&self) -> zbus::Result<String>;

    /// Profiles property
    #[zbus(property)]
    fn profiles(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Version property
    #[zbus(property)]
    fn version(&self) -> zbus::Result<String>;
}

#[derive(Debug, Clone, Default, Reactive)]
#[wrapper_type(PowerProfilesService)]
pub struct PowerProfilesServiceInner {
    #[property(get)]
    pub active_profile: String,
    #[property(get)]
    pub profiles: Vec<String>,

    #[property(get)]
    pub performance_degraded: bool,
    #[property(get)]
    pub performance_inhibited: bool,
}

reactive_wrapper!(pub PowerProfilesService<PowerProfilesServiceInner, Weak = WeakPowerProfilesServiceInner>);

impl PowerProfilesService {
    pub async fn new() -> Self {
        let this = Self {
            inner: Default::default(),
        };

        let Ok(proxy) = PowerProfilesProxy::new(&DBUS_SYSTEM_CONNECTION).await else {
            println!(
                "Failed to connect to power-profiles-daemon. Power profiles service will not be available."
            );
            return this;
        };

        let (active_profile, profiles, performance_degraded, performance_inhibited) = join!(
            proxy.active_profile(),
            proxy.profiles(),
            proxy.performance_degraded(),
            proxy.performance_inhibited()
        );
        this.inner.apply(|inner| {
            inner.active_profile = active_profile.unwrap_or_default();
            inner.profiles = profiles
                .unwrap_or_default()
                .iter()
                .map(|profile| {
                    let string: String = profile["Name"].clone().try_into().unwrap();
                    string
                })
                .collect();
            inner.performance_degraded = performance_degraded.unwrap_or_default() == "true";
            inner.performance_inhibited = performance_inhibited.unwrap_or_default() == "true";
        });

        let this2 = this.clone();
        gtk::glib::spawn_future_local(async move {
            let stream = proxy.receive_profiles_changed().await;
            // while stream.next().await.is_some() {
            //     let (active_profile, profiles, performance_degraded, performance_inhibited) = join!(
            //         proxy.active_profile(),
            //         proxy.profiles(),
            //         proxy.performance_degraded(),
            //         proxy.performance_inhibited()
            //     );
            //     this2.inner.apply(|inner| {
            //         inner.active_profile = active_profile.unwrap_or_default();
            //         inner.profiles = profiles
            //             .unwrap_or_default()
            //             .iter()
            //             .map(|profile| {
            //                 let string: String = profile["Name"].clone().try_into().unwrap();
            //                 string
            //             })
            //             .collect();
            //         inner.performance_degraded = performance_degraded.unwrap_or_default() == "true";
            //         inner.performance_inhibited = performance_inhibited.unwrap_or_default() == "true";
            //     });
            // }
        });

        this
    }
}
