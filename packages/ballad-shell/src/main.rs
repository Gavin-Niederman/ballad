use app::{AppControl, APP_CONTROL_SENDER};
use clap::Parser;
use smol::block_on;
use snafu::Snafu;
use zbus::interface;

mod app;
mod style;
mod utils;
mod widgets;

#[derive(Debug, Default, Clone, PartialEq, Eq, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(last = true)]
    gtk_args: Vec<String>,
}
#[derive(Debug, Default, Clone, PartialEq, Eq, clap::Subcommand)]
pub enum Command {
    #[default]
    #[command(visible_alias = "r")]
    Run,
    #[command(visible_alias = "q")]
    Quit,
}

pub struct BalladShell;
#[interface(
    name = "com.gavinniederman.BalladShell",
    proxy(
        default_service = "com.gavinniederman.BalladShell",
        default_path = "/com/gavinniederman/BalladShell",
    )
)]
impl BalladShell {
    fn quit(&self) {
        if let Some(sender) = APP_CONTROL_SENDER.get() {
            let _ = sender.try_send(AppControl::Quit);
        }
    }
}


fn main() -> Result<(), Error> {
    let args = Args::parse();
    let command = args.command.unwrap_or_default();
    block_on(smol::LocalExecutor::new().run(async {
        match command {
            Command::Run => {
                let _connection = zbus::connection::Builder::session()?
                    .name("com.gavinniederman.BalladShell")?
                    .serve_at("/com/gavinniederman/BalladShell", BalladShell)?
                    .build()
                    .await?;

                let res = app::launch_app(args.gtk_args)?;
                if res != gtk::glib::ExitCode::SUCCESS {
                    return Err(Error::Gtk { code: res });
                }
            }
            Command::Quit => {
                let dbus_connection = zbus::Connection::session().await?;
                let proxy = BalladShellProxy::new(&dbus_connection).await?;

                let res = proxy.quit().await;
                if res == Err(zbus::Error::InterfaceNotFound) {
                    return Err(Error::CloseFailed);
                } else {
                    res?;
                }
            }
        };

        Ok(())
    }))
}

#[derive(Snafu, Debug, Clone, PartialEq)]
pub enum Error {
    /// An application is already serving the ballad-shell DBus bus.
    /// This is likely because another instance of the shell is running.
    BusTaken,
    /// Failed to close a running ballad-shell instance.
    /// Is it running?
    CloseFailed,
    /// GTK exited with an error.
    #[snafu(display("GTK exited with error code: {}", code.value()))]
    Gtk { code: gtk::glib::ExitCode },
    #[snafu(transparent)]
    DBus { source: zbus::Error },
}
