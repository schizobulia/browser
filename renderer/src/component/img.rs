use std::collections::HashMap;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use network;

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
            Ok(data) => match image::load_from_memory(&data) {
                Ok(img) => {
                    let i = Image::from_dynamic(img, true, RenderAssetUsages::default());
                    img_bundle.image = UiImage {
                        texture: asset_server.add(i),
                        ..Default::default()
                    }
                }
                Err(e) => {
                    println!("Load img failed: {:?}", e);
                }
            },
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
