use std::{fs::read_dir, path::Path};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    bundle_resources();
    bundle_scss(out_dir);
}

fn bundle_resources() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/icons.gresource.xml",
        "icons.gresource",
    );

    println!("cargo:rerun-if-changed=resources");
}

fn bundle_scss(out_dir: &Path) {
    let style_files =
        read_dir("styles").expect("Failed to read styles directory. Does it exist locally?");
    let style_constants = style_files.fold(Vec::new(), |mut imports, file| {
        let file = file.unwrap();
        let path = file
            .path()
            .canonicalize()
            .expect("Failed to canonicalize style file path.");

        let constant_name = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .replace("-", "_")
            .to_ascii_uppercase();

        imports.push((constant_name, path.display().to_string()));
        imports
    });

    let scss_list = format!(
        "const SCSS_FILES: &[&str] = &[{}];",
        style_constants
            .iter()
            .fold(String::new(), |acc, (name, _)| acc + name + ", ")
    );
    let includes = style_constants
        .iter()
        .fold(String::new(), |acc, (name, path)| {
            let include = format!("pub const {name}: &str = include_str!(\"{path}\");\n");
            acc + &include
        });

    std::fs::write(
        out_dir.join("style_files.rs"),
        format!("{includes}\n{scss_list}"),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=styles");
}
