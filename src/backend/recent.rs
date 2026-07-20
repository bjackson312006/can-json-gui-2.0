//! Stores for the list of recently-opened files.
//! 
//! (aka we have a .txt file that we write the paths to)

use std::fs;
use std::path::{Path, PathBuf};

/// Directory name used under the OS config directory.
const APP_DIR: &str = "can-json-gui-2.0";

/// File the recent list is stored in.
const RECENT_FILE: &str = "recent.txt";

/// Maximum number of recent entries to keep.
const MAX_RECENT: usize = 10;

/// Full path to the on-disk recent-files store, if a config directory exists.
fn store_path() -> Option<PathBuf> {
    let mut dir = dirs::config_dir()?;
    dir.push(APP_DIR);
    Some(dir.join(RECENT_FILE))
}

/// Loads the recently-opened file paths.
pub fn load() -> Vec<PathBuf> {
    let Some(path) = store_path() else {
        return Vec::new();
    };
    let Ok(contents) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect()
}

/// Records `file` as the most-recently-opened file.
pub fn push(file: &Path) {
    let Some(store) = store_path() else {
        return;
    };

    let mut list = load();
    list.retain(|existing| existing != file);
    list.insert(0, file.to_path_buf());
    list.truncate(MAX_RECENT);

    if let Some(parent) = store.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let text = list
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("\n");
    let _ = fs::write(&store, text);
}

/// Removes `file` from the recent list, if present.
pub fn remove(file: &Path) {
    let Some(store) = store_path() else {
        return;
    };

    let mut list = load();
    let before = list.len();
    list.retain(|existing| existing != file);
    if list.len() == before {
        return; // nothing to remove; leave the store untouched
    }

    let text = list
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("\n");
    let _ = fs::write(&store, text);
}
