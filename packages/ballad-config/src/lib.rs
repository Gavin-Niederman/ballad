#[cfg(feature = "gtk")]
use gtk::glib;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "gtk",
    derive(glib::Enum),
    enum_type(name = "CatppuccinFlavor")
)]
pub enum CatppuccinFlavor {
    Frappe,
    #[default]
    Macchiato,
    Mocha,
    Latte,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed), boxed_type(name = "ThemeConfig"))]
pub struct ThemeConfig {
    pub catppuccin_flavor: CatppuccinFlavor,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed), boxed_type(name = "ShellConfig"))]
pub struct ShellConfig {
    pub theme: ThemeConfig,
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
