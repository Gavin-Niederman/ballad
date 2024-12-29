use ballad_config::CatppuccinFlavor;

include!(concat!(env!("OUT_DIR"), "/style_files.rs"));

const EXTRA_SCSS: &str = "
@use \"sass:math\";

$corner-radius: 16px;
$ui-radius: 8px;
$transition-length: 0.2s;
$transition: all $transition-length;
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FlavorVariables<'a> {
    rosewater: &'a str,
    flamingo: &'a str,
    pink: &'a str,
    mauve: &'a str,
    red: &'a str,
    maroon: &'a str,
    peach: &'a str,
    yellow: &'a str,
    green: &'a str,
    teal: &'a str,
    sky: &'a str,
    sapphire: &'a str,
    blue: &'a str,
    lavender: &'a str,

    text: &'a str,
    subtext_1: &'a str,
    subtext_0: &'a str,

    overlay_2: &'a str,
    overlay_1: &'a str,
    overlay_0: &'a str,

    surface_2: &'a str,
    surface_1: &'a str,
    surface_0: &'a str,

    base: &'a str,
    mantle: &'a str,
    crust: &'a str,
}

impl FlavorVariables<'_> {
    pub fn into_scss(
        Self {
            rosewater,
            flamingo,
            pink,
            mauve,
            red,
            maroon,
            peach,
            yellow,
            green,
            teal,
            sky,
            sapphire,
            blue,
            lavender,
            text,
            subtext_1,
            subtext_0,
            overlay_2,
            overlay_1,
            overlay_0,
            surface_2,
            surface_1,
            surface_0,
            base,
            mantle,
            crust,
        }: Self,
    ) -> String {
        format!(
            "
$rosewater: {rosewater};
$flamingo: {flamingo};
$pink: {pink};
$mauve: {mauve};
$red: {red};
$maroon: {maroon};
$peach: {peach};
$yellow: {yellow};
$green: {green};
$teal: {teal};
$sky: {sky};
$sapphire: {sapphire};
$blue: {blue};
$lavender: {lavender};

$text: {text};
$subtext-1: {subtext_1};
$subtext-0: {subtext_0};

$overlay-2: {overlay_2};
$overlay-1: {overlay_1};
$overlay-0: {overlay_0};

$surface-2: {surface_2};
$surface-1: {surface_1};
$surface-0: {surface_0};

$base: {base};
$mantle: {mantle};
$crust: {crust};"
        )
    }
}

impl From<CatppuccinFlavor> for FlavorVariables<'static> {
    fn from(flavor: CatppuccinFlavor) -> Self {
        match flavor {
            CatppuccinFlavor::Latte => LATTE,
            CatppuccinFlavor::Frappe => FRAPPE,
            CatppuccinFlavor::Macchiato => MACCHIATO,
            CatppuccinFlavor::Mocha => MOCHA,
        }
    }
}

const LATTE: FlavorVariables = FlavorVariables {
    rosewater: "#dc8a78",
    flamingo: "#dd7878",
    pink: "#ea76cb",
    mauve: "#8839ef",
    red: "#d20f39",
    maroon: "#e64553",
    peach: "#fe640b",
    yellow: "#df8e1d",
    green: "#40a02b",
    teal: "#179299",
    sky: "#04a5e5",
    sapphire: "#209fb5",
    blue: "#1e66f5",
    lavender: "#7287fd",

    text: "#4c4f69",
    subtext_0: "#6c6f85",
    subtext_1: "#5c5f77",

    overlay_2: "#7c7f93",
    overlay_1: "#8c8fa1",
    overlay_0: "#9ca0b0",

    surface_2: "#acb0be",
    surface_1: "#bcc0cc",
    surface_0: "#ccd0da",

    base: "#eff1f5",
    mantle: "#e6e9ef",
    crust: "#dce0e8",
};

const FRAPPE: FlavorVariables = FlavorVariables {
    rosewater: "#f2d5cf",
    flamingo: "#eebebe",
    pink: "#f4b8e4",
    mauve: "#ca9ee6",
    red: "#e78284",
    maroon: "#ea999c",
    peach: "#ef9f76",
    yellow: "#e5c890",
    green: "#a6d189",
    teal: "#81c8be",
    sky: "#99d1db",
    sapphire: "#85c1dc",
    blue: "#8caaee",
    lavender: "#babbf1",

    text: "#c6d0f5",
    subtext_1: "#b5bfe2",
    subtext_0: "#a5adce",

    overlay_2: "#949cbb",
    overlay_1: "#838ba7",
    overlay_0: "#737994",

    surface_2: "#626880",
    surface_1: "#51576d",
    surface_0: "#414559",

    base: "#303446",
    mantle: "#292c3c",
    crust: "#232634",
};

const MACCHIATO: FlavorVariables = FlavorVariables {
    rosewater: "#f4dbd6",
    flamingo: "#f0c6c6",
    pink: "#f5bde6",
    mauve: "#c6a0f6",
    red: "#ed8796",
    maroon: "#ee99a0",
    peach: "#f5a97f",
    yellow: "#eed49f",
    green: "#a6da95",
    teal: "#8bd5ca",
    sky: "#91d7e3",
    sapphire: "#7dc4e4",
    blue: "#8aadf4",
    lavender: "#b7bdf8",

    text: "#cad3f5",
    subtext_1: "#b8c0e0",
    subtext_0: "#a5adcb",

    overlay_2: "#939ab7",
    overlay_1: "#8087a2",
    overlay_0: "#6e738d",

    surface_2: "#5b6078",
    surface_1: "#494d64",
    surface_0: "#363a4f",

    base: "#24273a",
    mantle: "#1e2030",
    crust: "#181926",
};

const MOCHA: FlavorVariables = FlavorVariables {
    rosewater: "#f5e0dc",
    flamingo: "#f2cdcd",
    pink: "#f5c2e7",
    mauve: "#cba6f7",
    red: "#f38ba8",
    maroon: "#eba0ac",
    peach: "#fab387",
    yellow: "#f9e2af",
    green: "#a6e3a1",
    teal: "#94e2d5",
    sky: "#89dceb",
    sapphire: "#74c7ec",
    blue: "#89b4fa",
    lavender: "#b4befe",

    text: "#cdd6f4",
    subtext_1: "#bac2de",
    subtext_0: "#a6adc8",

    overlay_2: "#9399b2",
    overlay_1: "#7f849c",
    overlay_0: "#6c7086",

    surface_2: "#585b70",
    surface_1: "#45475a",
    surface_0: "#313244",

    base: "#1e1e2e",
    mantle: "#181825",
    crust: "#11111b",
};

fn scss_for_flavor(flavor: CatppuccinFlavor) -> String {
    let flavor_scss = FlavorVariables::into_scss(flavor.into());
    let real_styles = SCSS_FILES
        .iter()
        .fold(String::new(), |acc, &scss| acc + &format!("{}\n", scss));
    format!("{EXTRA_SCSS}\n{flavor_scss}\n{real_styles}")
}

pub fn compile_scss_for_flavor(flavor: CatppuccinFlavor) -> String {
    let scss = scss_for_flavor(flavor);
    grass::from_string(scss, &grass::Options::default()).unwrap()
}