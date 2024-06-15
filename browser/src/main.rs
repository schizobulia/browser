use std::env;

use ui;

fn main() {
    #[cfg(not(debug_assertions))]
    init_bevy_asset_path();

    let _ = ui::open_window();
}
#[allow(dead_code)]
fn init_bevy_asset_path() {
    match env::current_exe() {
        Ok(path) => {
            if let Some(exe) = path.parent() {
                if let Some(resources) = exe.parent() {
                    let dir = resources.join("Resources");
                    env::set_var("BEVY_ASSET_ROOT", dir);
                };
            };
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };
}
