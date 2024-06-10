use std::{env, path::PathBuf};

pub fn get_asset_network_path() -> Option<PathBuf> {
    match env::current_dir() {
        Ok(path) => Some(path.join("browser").join("assets").join("network")),
        Err(_) => None,
    }
}
