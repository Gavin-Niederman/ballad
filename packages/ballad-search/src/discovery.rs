use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use freedesktop_desktop_entry as fd_entry;
use smol::stream::StreamExt;

static LOCALES: LazyLock<Vec<String>> = LazyLock::new(fd_entry::get_languages_from_env);

/// Represents an application parsed from a .desktop file that can be launched.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Application {
    /// The path of the .desktop file.
    pub path: PathBuf,

    /// The name of the application.
    pub name: String,
    /// An optional description of the application.
    pub description: Option<String>,
    /// The command to execute the application.
    /// This should be run in a shell.
    pub exec: String,
    /// An optional icon name or path for the application.
    pub icon: Option<String>,
}
impl Application {
    pub async fn parse_file(path: impl AsRef<Path>, locales: &[String]) -> Option<Self> {
        let content = smol::fs::read_to_string(&path).await.ok()?;
        Self::parse(&content, path, locales)
    }

    pub fn parse(
        desktop: impl AsRef<str>,
        path: impl AsRef<Path>,
        locales: &[String],
    ) -> Option<Self> {
        let path = path.as_ref();
        let entry = fd_entry::DesktopEntry::from_str(path, desktop.as_ref(), Some(locales)).ok()?;

        let name = entry.name(locales)?;
        let description = entry.comment(locales);
        let exec = entry.exec()?;
        let icon = entry.icon();

        Some(Self {
            path: path.to_path_buf(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            exec: exec.to_string(),
            icon: icon.map(|s| s.to_string()),
        })
    }
}

fn application_dirs() -> Option<impl Iterator<Item = PathBuf>> {
    Some(
        xdg::BaseDirectories::new()
            .ok()?
            .list_data_files("applications")
            .into_iter(),
    )
}

pub async fn applications() -> impl Iterator<Item = Application> {
    let mut execs = HashSet::new();
    smol::stream::iter(application_dirs().into_iter().flatten())
        .then(|path| async move { Application::parse_file(path, &LOCALES).await })
        .filter_map(|app| {
            if let Some(app) = app {
                let exec = app.exec.clone();
                if !execs.contains(&exec) {
                    execs.insert(exec);
                    Some(app)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .await
        .into_iter()
}

pub async fn locate(query: &str) -> impl Iterator<Item = PathBuf> {
    let output = smol::process::Command::new("locate")
        .args(["-il", "15", query])
        .output()
        .await
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned());
    output
        .map(|output| {
            output
                .lines()
                .map(|string| string.to_string())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .flatten()
        .map(PathBuf::from)
}
