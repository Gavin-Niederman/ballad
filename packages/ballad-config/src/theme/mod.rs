mod builtin_themes;

use builtin_themes::{catppuccin_latte, catppuccin_macchiato};
use gtk::glib;
use serde::{Deserialize, Serialize};

use crate::get_or_init_shell_config;

/// The color variables for the theme
/// These variables are designed for interop with catppuccin themes, so do your best with the required colors if you are using a different theme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "BalladConfigThemeColors"))]
pub struct ThemeColors {
    pub pink: String,
    pub orange: String,
    pub red: String,
    pub yellow: String,
    pub green: String,
    pub blue: String,
    pub purple: String,

    pub text: String,
    pub subtext_1: String,
    pub subtext_0: String,

    pub overlay_2: String,
    pub overlay_1: String,
    pub overlay_0: String,

    pub surface_2: String,
    pub surface_1: String,
    pub surface_0: String,

    /// Background color for the lowest layer of an application
    pub bg_0: String,
    /// Background color for the middle (second) layer of an application
    pub bg_1: String,
    /// Background color for the top layer of an application
    pub bg_2: String,
}

impl ThemeColors {
    pub fn as_scss(&self) -> String {
        let Self {
            pink,
            orange,
            red,
            yellow,
            green,
            blue,
            purple,
            text,
            subtext_1,
            subtext_0,
            overlay_2,
            overlay_1,
            overlay_0,
            surface_2,
            surface_1,
            surface_0,
            bg_0,
            bg_1,
            bg_2,
        } = self;

        format!(
            "$pink: {pink};
$orange: {orange};
$red: {red};
$yellow: {yellow};
$green: {green};
$blue: {blue};
$blue: {blue};
$purple: {purple};

$text: {text};
$subtext-1: {subtext_1};
$subtext-0: {subtext_0};

$overlay-2: {overlay_2};
$overlay-1: {overlay_1};
$overlay-0: {overlay_0};

$surface-2: {surface_2};
$surface-1: {surface_1};
$surface-0: {surface_0};

$bg_0: {bg_0};
$bg_1: {bg_1};
$bg_2: {bg_2};"
        )
    }
}

/// The variant of the theme (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Enum, glib::Variant))]
#[cfg_attr(feature = "gtk", enum_type(name = "BalladConfigThemeVariant"))]
pub enum ThemeVariant {
    Light,
    Dark,
}
impl ThemeVariant {
    pub fn is_light(&self) -> bool {
        matches!(self, ThemeVariant::Light)
    }
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "BalladConfigCustomTheme"))]
pub struct Theme {
    pub colors: ThemeColors,
    pub name: String,
    pub variant: ThemeVariant,
    pub gtk_theme: Option<String>,
}
impl Theme {
    pub fn is_light(&self) -> bool {
        self.variant.is_light()
    }
    pub fn is_dark(&self) -> bool {
        self.variant.is_dark()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "BalladConfigThemeSelection"))]
// We cant box the custom variant while implementing ToVariant
#[allow(clippy::large_enum_variant)]
pub enum ThemeSelection {
    CatppuccinFrappe,
    #[default]
    CatppuccinMacchiato,
    CatppuccinMocha,
    CatppuccinLatte,
    Custom(String),
}

impl ThemeSelection {
    fn try_read_custom_theme(&self) -> Option<Theme> {
        match self {
            Self::Custom(theme_name) => {
                let config = get_or_init_shell_config();
                let custom_themes = config.theme.custom_themes;

                custom_themes
                    .into_iter()
                    .find(|theme| theme.name == *theme_name)
            }
            _ => panic!("Cannot read custom theme from non-custom theme selection"),
        }
    }

    pub fn is_light(&self) -> Option<bool> {
        Some(match self {
            Self::CatppuccinLatte => true,
            Self::Custom(_) => {
                let theme = self.try_read_custom_theme()?;
                theme.variant.is_light()
            }
            _ => false,
        })
    }
    pub fn is_dark(&self) -> Option<bool> {
        self.is_light().map(|light| !light)
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(match self {
            ThemeSelection::CatppuccinFrappe => builtin_themes::catppuccin_frappe(),
            ThemeSelection::CatppuccinMacchiato => builtin_themes::catppuccin_macchiato(),
            ThemeSelection::CatppuccinMocha => builtin_themes::catppuccin_mocha(),
            ThemeSelection::CatppuccinLatte => builtin_themes::catppuccin_latte(),
            ThemeSelection::Custom(_) => return self.try_read_custom_theme(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "gtk", derive(glib::Boxed, glib::Variant))]
#[cfg_attr(feature = "gtk", boxed_type(name = "BalladConfigThemeConfig"))]
pub struct ThemeConfig {
    pub selected_theme: ThemeSelection,
    pub custom_themes: Vec<Theme>,
    pub default_dark_gtk_theme: String,
    pub default_light_gtk_theme: String,

    pub corner_radius: f64,
    pub ui_radius: u32,
    pub transition_length: f64,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            selected_theme: Default::default(),
            custom_themes: Default::default(),
            default_dark_gtk_theme: catppuccin_macchiato().gtk_theme.unwrap(),
            default_light_gtk_theme: catppuccin_latte().gtk_theme.unwrap(),

            corner_radius: 16.0,
            ui_radius: 8,
            transition_length: 0.2,
        }
    }
}

impl ThemeConfig {
    pub fn as_scss(&self) -> Option<String> {
        let Self {
            selected_theme,
            custom_themes: _,
            default_dark_gtk_theme: _,
            default_light_gtk_theme: _,
            corner_radius,
            ui_radius,
            transition_length,
        } = self;

        let colors = selected_theme.theme()?.colors;
        let colors_scss = colors.as_scss();

        Some(format!(
            "{colors_scss}

$corner-radius: {corner_radius}px;
$ui-radius: {ui_radius}px;
$transition-length: {transition_length}s;
$transition: all $transition-length;",
        ))
    }
}

pub fn get_or_init_all_theme_selections() -> Vec<ThemeSelection> {
    let config = get_or_init_shell_config();
    let mut themes = vec![
        ThemeSelection::CatppuccinFrappe,
        ThemeSelection::CatppuccinMacchiato,
        ThemeSelection::CatppuccinMocha,
        ThemeSelection::CatppuccinLatte,
    ];
    themes.extend(
        config
            .theme
            .custom_themes
            .into_iter()
            .map(|theme| ThemeSelection::Custom(theme.name)),
    );
    themes
}
