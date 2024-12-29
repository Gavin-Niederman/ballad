use std::{thread::sleep, time::Duration};

use ballad_config::{ShellConfig, ThemeConfig};

fn main() {
    let config = ballad_config::get_or_init_shell_config();
    println!("Config: {:?}", config);
    ballad_config::set_shell_config(&Default::default());
    sleep(Duration::from_secs(1));
    ballad_config::set_shell_config(&ShellConfig {
        theme: ThemeConfig {
            catppuccin_flavor: ballad_config::CatppuccinFlavor::Latte,
        }
    });
}
