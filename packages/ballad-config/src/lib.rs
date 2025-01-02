#[cfg(feature = "gtk")]
use gtk::glib;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Enum, glib::Variant))]
#[cfg_attr(feature = "gtk", enum_type(name = "CatppuccinFlavor"))]
pub enum CatppuccinFlavor {
    Frappe,
    #[default]
    Macchiato,
    Mocha,
    Latte,
}

impl CatppuccinFlavor {
    pub fn is_light(&self) -> bool {
        matches!(self, CatppuccinFlavor::Latte)
    }
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "ThemeConfig"))]
pub struct ThemeConfig {
    pub catppuccin_flavor: CatppuccinFlavor,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Enum, glib::Variant))]
#[cfg_attr(feature = "gtk", enum_type(name = "PowerProfile"))]
pub enum PowerProfile {
    #[default]
    Balanced,
    HighPerformance,
    PowerSaver,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "PowerProfilesConfig"))]
pub struct PowerProfilesConfig {
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "ShellConfig"))]
pub struct ShellConfig {
    pub theme: ThemeConfig,
    pub power_profiles: PowerProfilesConfig,
}

pub fn shell_config_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("ballad")
        .unwrap()
        .place_config_file("shell_config.toml")
        .unwrap()
}

pub fn get_or_init_shell_config() -> ShellConfig {
    let path = shell_config_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        let config = ShellConfig::default();
        std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        config
    }
}
pub fn set_shell_config(config: &ShellConfig) {
    let path = shell_config_path();
    std::fs::write(&path, toml::to_string(config).unwrap()).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "ServiceConfig"))]
pub struct ServiceConfig {
    pub poll_interval_millis: u32,
}
impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            poll_interval_millis: 10,
        }
    }
}

pub fn service_config_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("ballad")
        .unwrap()
        .place_config_file("service_config.toml")
        .unwrap()
}

pub fn get_or_init_service_config() -> ServiceConfig {
    let path = service_config_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        let config = ServiceConfig::default();
        std::fs::write(&path, toml::to_string(&config).unwrap()).unwrap();
        config
    }
}

pub fn set_service_config(config: &ServiceConfig) {
    let path = service_config_path();
    std::fs::write(&path, toml::to_string(config).unwrap()).unwrap();
}
