use std::sync::LazyLock;

pub mod audio;
pub mod config;
pub mod niri;
pub mod upower;
pub mod variable;
pub mod accounts;

pub(crate) static DBUS_SYSTEM_CONNECTION: LazyLock<zbus::Connection> =
    LazyLock::new(|| smol::block_on(zbus::Connection::system()).unwrap());
