use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use url::Url;

pub mod cache;
pub mod discovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, Serialize, Deserialize)]
pub enum SearchEngine {
    #[default]
    DuckDuckGo,
    Google,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SearchResult {
    Application { path: PathBuf },
    File { path: PathBuf },
    WebSearch { query: String },
    Website { url: Url },
}

pub async fn applications_results() -> Vec<SearchResult> {
    discovery::applications()
        .await
        .map(|app| SearchResult::Application { path: app.path })
        .collect()
}
pub async fn files_results(query: impl AsRef<str>) -> Vec<SearchResult> {
    discovery::locate(query.as_ref())
        .await
        .map(|path| SearchResult::File { path })
        .collect()
}
