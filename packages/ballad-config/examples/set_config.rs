use std::{thread::sleep, time::Duration};

use ballad_config::{ServiceConfig, ShellConfig, ThemeConfig};

fn main() {
    let config = ballad_config::get_or_init_shell_config();
    let service_config = ballad_config::get_or_init_service_config();
    println!("Config: {:?}, Service {:?}", config, service_config);
    loop {
        ballad_config::set_service_config(&ServiceConfig {
            poll_interval_millis: 1000,
        });
        println!("1000");
        ballad_config::set_shell_config(&ShellConfig {
            theme: ThemeConfig {
                catppuccin_flavor: ballad_config::CatppuccinFlavor::Latte,
            },
            ..config
        });
        sleep(Duration::from_secs(1));
        ballad_config::set_shell_config(&ShellConfig {
            theme: ThemeConfig {
                catppuccin_flavor: ballad_config::CatppuccinFlavor::Frappe,
            },
            ..config
        });
        sleep(Duration::from_secs(1));
        ballad_config::set_service_config(&ServiceConfig {
            poll_interval_millis: 1,
        });
        println!("1");
        ballad_config::set_shell_config(&ShellConfig {
            theme: ThemeConfig {
                catppuccin_flavor: ballad_config::CatppuccinFlavor::Macchiato,
            },
            ..config
        });
        sleep(Duration::from_secs(1));
        ballad_config::set_shell_config(&ShellConfig {
            theme: ThemeConfig {
                catppuccin_flavor: ballad_config::CatppuccinFlavor::Mocha,
            },
            ..config
        });
        sleep(Duration::from_secs(1));
    }
}
