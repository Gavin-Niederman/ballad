use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchCache {
    /// Map of search results to their UUIDs
    pub result_map: HashMap<Uuid, SearchResult>,
    /// Map of search result UUIDs to their frequency of use
    pub frequency_map: HashMap<Uuid, u32>,
}

pub fn search_cache_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("ballad")
        .unwrap()
        .place_cache_file("search_cache.toml")
        .unwrap()
}

pub fn get_or_init_search_cache() -> SearchCache {
    let path = search_cache_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap();
        toml::from_str(&content).unwrap()
    } else {
        let cache = SearchCache::default();
        std::fs::write(&path, toml::to_string(&cache).unwrap()).unwrap();
        cache
    }
}

pub fn set_search_cache(cache: &SearchCache) {
    let path = search_cache_path();
    std::fs::write(path, toml::to_string(cache).unwrap()).unwrap();
}
