use std::sync::LazyLock;

pub mod accounts;
pub mod audio;
pub mod brightness;
pub mod config;
pub mod niri;
pub mod upower;
pub mod reactive;

pub(crate) static DBUS_SYSTEM_CONNECTION: LazyLock<zbus::Connection> =
    LazyLock::new(|| smol::block_on(zbus::Connection::system()).unwrap());
