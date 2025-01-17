use ballad_config::ThemeConfig;

include!(concat!(env!("OUT_DIR"), "/style_files.rs"));

fn scss_for_config(config: &ThemeConfig) -> Option<String> {
    let flavor_scss = config.as_scss()?;

    let real_styles = SCSS_FILES
        .iter()
        .fold(String::new(), |acc, &scss| acc + &format!("{}\n", scss));

    Some(format!(
        "@use \"sass:math\";\n{}\n{}",
        flavor_scss, real_styles
    ))
}

pub fn compile_scss_for_config(flavor: &ThemeConfig) -> Option<String> {
    let scss = scss_for_config(flavor)?;
    Some(grass::from_string(scss, &grass::Options::default()).unwrap())
}
