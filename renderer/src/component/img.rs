use std::collections::HashMap;

use bevy::prelude::*;
use cache::get_asset_network_path;
use network;
use std::fs;

pub fn init_style() {}

pub fn add_img_component(
    parent_id: Entity,
    commands: &mut Commands,
    attributes: HashMap<String, String>,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let mut img_bundle = ImageBundle {
        style: Style {
            ..Default::default()
        },
        ..Default::default()
    };
    if let Some(src) = attributes.get("src") {
        match network::get_img_by_url(src.clone()) {
            Ok(data) => {
                if let Some(p) = get_asset_network_path() {
                    // todo: The suffix name should be obtained dynamically
                    let digest = format!("{:x}.jpg", md5::compute(src.clone()));
                    match fs::write(p.join(digest.clone()), data) {
                        Ok(_) => {
                            let mut img_path = String::new();
                            img_path.push_str("network/");
                            img_path.push_str(digest.as_str());
                            img_bundle.image = UiImage {
                                texture: asset_server.load(img_path),
                                ..Default::default()
                            };
                        }
                        Err(e) => {
                            println!("The image could not be created: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Get img failed: {:?}", e);
            }
        };
    };

    let childern_id = commands.spawn(img_bundle).id();
    commands
        .entity(parent_id)
        .push_children(&vec![childern_id.clone()]);
    childern_id
}
