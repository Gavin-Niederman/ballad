use clap::Parser;

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

fn main() {
    let args = Args::parse();
    let command = args.command.unwrap_or_default();
    match command {
        Command::Run => {
            app::launch_app(args.gtk_args);
        }
        Command::Quit => {}
    };
}
