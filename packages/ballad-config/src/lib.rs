pub mod theme;

#[cfg(feature = "gtk")]
use gtk::glib;
use snafu::Snafu;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub use theme::{ThemeConfig, ThemeSelection};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

pub fn get_or_init_shell_config() -> Result<ShellConfig, Error> {
    let path = shell_config_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    Ok(if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        toml::from_str(&content)?
    } else {
        let config = ShellConfig::default();
        std::fs::write(&path, toml::to_string(&config)?)?;
        config
    })
}
pub fn set_shell_config(config: &ShellConfig) -> Result<(), Error> {
    let path = shell_config_path();
    std::fs::write(&path, toml::to_string(config)?)?;

    Ok(())
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

pub fn get_or_init_service_config() -> Result<ServiceConfig, Error> {
    let path = service_config_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    Ok(if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        toml::from_str(&content)?
    } else {
        let config = ServiceConfig::default();
        std::fs::write(&path, toml::to_string(&config)?)?;
        config
    })
}

pub fn set_service_config(config: &ServiceConfig) -> Result<(), Error> {
    let path = service_config_path();
    Ok(std::fs::write(&path, toml::to_string(config)?)?)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Io { source: std::io::Error },
    #[snafu(transparent)]
    TomlDeserialize { source: toml::de::Error },
    #[snafu(transparent)]
    TomlSerialize { source: toml::ser::Error },
}
