use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use image::DynamicImage;
use multimedia_processing::video::GstPlayer;
use scraper::ElementRef;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum VideoState {
    Init,
    Playing,
    Paused,
    Start,
    Ready,
    Stop,
}

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct VideoPlayer {
    state: VideoState,
    pub timer: Arc<Mutex<Timer>>,
    pub id: Option<Entity>,
    pub width: f32,
    pub height: f32,
    pub uri: String,
    pub pipeline: Option<GstPlayer>,
    autoplay: bool,
}

pub fn add_video_component(
    parent_id: Entity,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    attributes: HashMap<String, String>,
) -> Entity {
    let default_size = Vec2::new(200.0, 200.0);
    let bundle = ImageBundle {
        image: UiImage {
            texture: asset_server.load("network/4d483daacb2dedbbafb1e53d07e22199.jpg"),
            ..Default::default()
        },
        style: Style {
            width: Val::Px(default_size.x),
            height: Val::Px(default_size.y),
            ..Default::default()
        },
        ..Default::default()
    };

    let childern_id = commands.spawn(bundle).id();
    commands
        .entity(parent_id)
        .push_children(&vec![childern_id.clone()]);

    if let Some(url) = attributes.get("src") {
        commands.entity(childern_id).insert(init_video_player(
            url.clone(),
            default_size,
            childern_id,
            attributes,
        ));
    }
    childern_id
}
/**
 * get url and type from source element
 */
pub fn get_source_element(element: ElementRef) -> HashMap<String, String> {
    let mut attr = HashMap::new();
    for child in element.children().rev() {
        if let Some(child_element) = ElementRef::wrap(child) {
            let item = child_element.value();
            if item.name() == "source" {
                item.attrs.iter().for_each(|x| {
                    attr.insert(x.0.local.to_string(), x.1.to_string());
                });
            }
        }
    }
    attr
}

fn init_video_player(
    url: String,
    default_size: Vec2,
    id: Entity,
    attributes: HashMap<String, String>,
) -> VideoPlayer {
    let mut autoplay = false;
    if let Some(tag) = attributes.get("autoplay") {
        if tag == "autoplay" {
            autoplay = true;
        }
    }
    VideoPlayer {
        uri: url.clone(),
        state: VideoState::Init,
        timer: Arc::new(Mutex::new(Timer::from_seconds(1.0, TimerMode::Repeating))),
        width: default_size.x,
        height: default_size.y,
        id: Some(id),
        pipeline: None,
        autoplay: autoplay,
    }
}

pub fn handle_playing_state(
    video_player: &mut VideoPlayer,
    image_handle: &mut UiImage,
    images: &mut Assets<Image>,
    time: &Res<Time>,
) {
    if let Ok(mut player_time) = video_player.timer.lock() {
        if player_time.tick(time.delta()).just_finished() {
            if let Some(ref_pipeline) = video_player.pipeline.as_ref() {
                if let Ok(mut frames) = ref_pipeline.frame.lock() {
                    if let Some(data) = frames.pop_front() {
                        if let Some(rbg_data) =
                            image::RgbaImage::from_raw(data.width, data.height, data.data)
                        {
                            let canvas = Image::from_dynamic(
                                DynamicImage::ImageRgba8(rbg_data),
                                true,
                                RenderAssetUsages::default(),
                            );
                            image_handle.texture = images.add(canvas);
                            if let Ok(mut pts) = ref_pipeline.previous_pts.lock() {
                                let dt = (data.pts - *pts) / 1_000_000;
                                player_time.set_duration(Duration::from_millis(dt));
                                *pts = data.pts;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub fn render_video_frame(
    mut query: Query<(&mut VideoPlayer, &mut UiImage)>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
) {
    for (mut video_player, mut image_handle) in query.iter_mut() {
        match video_player.state {
            VideoState::Playing => {
                handle_playing_state(&mut video_player, &mut image_handle, &mut images, &time)
            }
            VideoState::Init => {
                if video_player.id.is_some() {
                    video_player.state = VideoState::Ready;
                    initialize_video_player(&mut video_player);
                }
            }
            VideoState::Start => {
                video_player.state = VideoState::Playing;
                if let Some(video_player) = video_player.pipeline.as_ref() {
                    video_player.play();
                }
            }
            VideoState::Paused => {
                if let Some(video_player) = video_player.pipeline.as_ref() {
                    video_player.pause();
                }
            }
            VideoState::Stop => {
                if let Some(video_player) = video_player.pipeline.as_ref() {
                    video_player.destroy();
                }
            }
            _ => {}
        }
    }
}

fn initialize_video_player(video_player: &mut VideoPlayer) {
    let pipeline = GstPlayer::new(video_player.uri.as_str());
    let pipeline_clone = Arc::new(Mutex::new(pipeline.clone()));
    thread::spawn(move || {
        if let Ok(mut pipeline) = pipeline_clone.lock() {
            pipeline.start();
        }
    });
    video_player.pipeline = Some(pipeline);
    if video_player.autoplay {
        video_player.state = VideoState::Start;
    }
}
