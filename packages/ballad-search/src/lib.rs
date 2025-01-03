use std::path::PathBuf;

pub mod cache;

fn application_dirs() -> Option<impl Iterator<Item = PathBuf>> {
    Some(
        xdg::BaseDirectories::new()
            .ok()?
            .find_data_files("applications"),
    )
}
