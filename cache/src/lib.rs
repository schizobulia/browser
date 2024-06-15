use std::{env, path::PathBuf};

#[cfg(debug_assertions)]
pub fn get_asset_network_path() -> Option<PathBuf> {
    match env::current_dir() {
        Ok(path) => Some(path.join("browser").join("assets").join("network")),
        Err(_) => None,
    }
}

#[cfg(not(debug_assertions))]
pub fn get_asset_network_path() -> Option<PathBuf> {
    match env::var("BEVY_ASSET_ROOT") {
        Ok(path) => {
            let p = std::path::Path::new(&path).join("assets").join("network");
            Some(p)
        }
        Err(_) => None,
    }
}
