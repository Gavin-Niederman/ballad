use super::{Theme, ThemeColors, ThemeVariant};

pub fn catppuccin_latte() -> Theme {
    Theme {
        colors: ThemeColors {
            pink: "#ea76cb".to_string(),
            orange: "#fe640b".to_string(),
            red: "#d20f39".to_string(),
            yellow: "#df8e1d".to_string(),
            green: "#40a02b".to_string(),
            blue: "#04a5e5".to_string(),
            purple: "#7287fd".to_string(),
            text: "#4c4f69".to_string(),
            subtext_0: "#6c6f85".to_string(),
            subtext_1: "#5c5f77".to_string(),
            overlay_2: "#7c7f93".to_string(),
            overlay_1: "#8c8fa1".to_string(),
            overlay_0: "#9ca0b0".to_string(),
            surface_2: "#acb0be".to_string(),
            surface_1: "#bcc0cc".to_string(),
            surface_0: "#ccd0da".to_string(),
            bg_0: "#eff1f5".to_string(),
            bg_1: "#e6e9ef".to_string(),
            bg_2: "#dce0e8".to_string(),
        },
        variant: (ThemeVariant::Light),
        name: "Catppuccin Latte".to_string(),
        gtk_theme: Some("catppuccin-latte-sky-standard".to_string())
    }
}

pub fn catppuccin_frappe() -> Theme {
    Theme {
        colors: ThemeColors {
            pink: "#f4b8e4".to_string(),
            orange: "#ef9f76".to_string(),
            red: "#e78284".to_string(),
            yellow: "#e5c890".to_string(),
            green: "#a6d189".to_string(),
            blue: "#99d1db".to_string(),
            purple: "#babbf1".to_string(),
            text: "#c6d0f5".to_string(),
            subtext_0: "#a5adce".to_string(),
            subtext_1: "#b5bfe2".to_string(),
            overlay_2: "#949cbb".to_string(),
            overlay_1: "#838ba7".to_string(),
            overlay_0: "#737994".to_string(),
            surface_2: "#626880".to_string(),
            surface_1: "#51576d".to_string(),
            surface_0: "#414559".to_string(),
            bg_0: "#303446".to_string(),
            bg_1: "#292c3c".to_string(),
            bg_2: "#232634".to_string(),
        },
        variant: (ThemeVariant::Dark),
        name: "Catppuccin Frappé".to_string(),
        gtk_theme: Some("catppuccin-frappe-sky-standard".to_string())
    }
}

pub fn catppuccin_macchiato() -> Theme {
    Theme {
        colors: ThemeColors {
            pink: "#f5bde6".to_string(),
            orange: "#f5a97f".to_string(),
            red: "#ed8796".to_string(),
            yellow: "#eed49f".to_string(),
            green: "#a6da95".to_string(),
            blue: "#91d7e3".to_string(),
            purple: "#b7bdf8".to_string(),
            text: "#cad3f5".to_string(),
            subtext_0: "#a5adcb".to_string(),
            subtext_1: "#b8c0e0".to_string(),
            overlay_2: "#939ab7".to_string(),
            overlay_1: "#8087a2".to_string(),
            overlay_0: "#6e738d".to_string(),
            surface_2: "#5b6078".to_string(),
            surface_1: "#494d64".to_string(),
            surface_0: "#363a4f".to_string(),
            bg_0: "#24273a".to_string(),
            bg_1: "#1e2030".to_string(),
            bg_2: "#181926".to_string(),
        },
        variant: (ThemeVariant::Dark),
        name: "Catppuccin Macchiato".to_string(),
        gtk_theme: Some("catppuccin-macchiato-sky-standard".to_string())
    }
}

pub fn catppuccin_mocha() -> Theme {
    Theme {
        colors: ThemeColors {
            pink: "#f5c2e7".to_string(),
            orange: "#fab387".to_string(),
            red: "#f38ba8".to_string(),
            yellow: "#f9e2af".to_string(),
            green: "#a6e3a1".to_string(),
            blue: "#89dceb".to_string(),
            purple: "#b4befe".to_string(),
            text: "#cdd6f4".to_string(),
            subtext_0: "#a6adc8".to_string(),
            subtext_1: "#bac2de".to_string(),
            overlay_2: "#9399b2".to_string(),
            overlay_1: "#7f849c".to_string(),
            overlay_0: "#6c7086".to_string(),
            surface_2: "#585b70".to_string(),
            surface_1: "#45475a".to_string(),
            surface_0: "#313244".to_string(),
            bg_0: "#1e1e2e".to_string(),
            bg_1: "#181825".to_string(),
            bg_2: "#11111b".to_string(),
        },
        variant: (ThemeVariant::Dark),
        name: "Catppuccin Mocha".to_string(),
        gtk_theme: Some("catppuccin-mocha-sky-standard".to_string())
    }
}
