use std::{thread::sleep, time::Duration};

use ballad_config::{theme::{Theme, ThemeColors}, ServiceConfig, ShellConfig, ThemeConfig};

fn main() {
    let theme = Theme {
        colors: ThemeColors {
            pink: "#d3869b".to_string(),
            orange: "#d65d0e".to_string(),
            red: "#cc241d".to_string(),
            yellow: "#d79921".to_string(),
            green: "#98971a".to_string(),
            blue: "#458588".to_string(),
            purple: "#b16286".to_string(),
            text: "#ebdbb2".to_string(),
            subtext_1: "#ebdbb2".to_string(),
            subtext_0: "#d5c4a1".to_string(),
            overlay_2: "#665c54".to_string(),
            overlay_1: "#7c6f64".to_string(),
            overlay_0: "#928374".to_string(),
            surface_2: "#3c3836".to_string(),
            surface_1: "#282828".to_string(),
            surface_0: "#282828".to_string(),
            bg_0: "#1d2021".to_string(),
            bg_1: "#282828".to_string(),
            bg_2: "#32302f".to_string(),
        },
        name: "Gruvbox".to_string(),
        variant: ballad_config::theme::ThemeVariant::Dark,
        gtk_theme: None,
    };

    println!("{}", toml::to_string_pretty(&theme).unwrap());
}
