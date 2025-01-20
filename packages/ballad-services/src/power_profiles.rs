use std::cell::LazyCell;

use ballad_macro::Reactive;
use futures::join;
use smol::stream::StreamExt;
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

    proxy: Option<PowerProfilesProxy<'static>>,
}

impl PowerProfilesServiceInner {
    pub async fn update(&mut self) {
        if let Some(proxy) = self.proxy.as_ref() {
            let (active_profile, profiles, performance_degraded, performance_inhibited) = join!(
                proxy.active_profile(),
                proxy.profiles(),
                proxy.performance_degraded(),
                proxy.performance_inhibited()
            );

            self.active_profile = active_profile.unwrap_or_default();
            self.profiles = profiles
                .unwrap_or_default()
                .iter()
                .map(|profile| {
                    let string: String = profile["Profile"].clone().try_into().unwrap();
                    string
                })
                .collect();
            self.performance_degraded = performance_degraded.unwrap_or_default() == "true";
            self.performance_inhibited = performance_inhibited.unwrap_or_default() == "true";
        }
    }
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

        this.inner.apply(|inner| {
            inner.proxy = Some(proxy);

            smol::block_on(inner.update())
        });     

        let this2 = this.clone();
        gtk::glib::spawn_future_local(async move {
            let proxy = zbus::fdo::PropertiesProxy::new(&DBUS_SYSTEM_CONNECTION, "org.freedesktop.UPower.PowerProfiles", "/org/freedesktop/UPower/PowerProfiles")
                .await
                .unwrap();
            
            let mut stream = proxy.receive_properties_changed().await.unwrap();
            while stream.next().await.is_some() {
                let mut inner = this2.inner.get().await;
                inner.update().await;
                this2.inner.set(inner).await;
            }
        });

        this
    }

    pub fn set_active_profile(&self, profile: String) {
        self.inner.apply(|inner| {
            inner.active_profile = profile;
        });
    }
}

impl Default for PowerProfilesService {
    fn default() -> Self {
        smol::block_on(Self::new())
    }
}

thread_local! {
    pub static POWER_PROFILES_SERVICE: LazyCell<PowerProfilesService> = LazyCell::new(PowerProfilesService::default)
}
