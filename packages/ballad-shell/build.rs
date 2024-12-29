fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/icons.gresource.xml",
        "icons.gresource",
    );

    println!("cargo:rerun-if-changed=resources");
}
