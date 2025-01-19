use std::sync::LazyLock;

pub mod accounts;
pub mod audio;
pub mod brightness;
pub mod config;
pub mod niri;
pub mod reactive;
pub mod upower;

pub(crate) static DBUS_SYSTEM_CONNECTION: LazyLock<zbus::Connection> =
    LazyLock::new(|| smol::block_on(zbus::Connection::system()).unwrap());
