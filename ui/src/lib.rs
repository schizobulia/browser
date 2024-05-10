use bean::{qaq, ui_state::UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use network::get_html_by_url;
use renderer::{render_document, update_document_by_action};

pub fn open_window() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, start_up)
        // .add_systems(PreUpdate, pre_update)
        .add_systems(Update, (update, update_document_by_action))
        // .add_systems(PostUpdate, post_update)
        // .add_systems(Last, last)
        .run();
}

/**
 * Executes when the application starts.
 * This phase is typically used to initialize resources, set state, and configure.
 */
fn start_up(
    commands: Commands,
    asset_server: Res<AssetServer>,
    ui_state: ResMut<UiState>,
) {
    init_render_document(ui_state, commands, asset_server);
}

fn init_render_document(
    ui_state: ResMut<UiState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
    let res = get_html_by_url(ui_state.name.clone());
    match res {
        Ok(html) => {
            render_document(commands, asset_server, ui_state, html);
        }
        Err(e) => {
            println!("Get html failed: {:?}", e);
        }
    }
}

/**
 * Execute before the update phase.
 * This phase can be used to deal with systems that need to run before the main update logic.
 */
// fn pre_update(mut commands: Commands, asset_server: Res<AssetServer>) {}

/**
 * Executes when each frame is updated.
 * This is the main stage of implementing logic.
 */
fn update(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    query: Query<Entity>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("Top panel")
        .exact_height(15.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut ui_state.name);
                // 添加一个刷新按钮
                if ui.button("Refresh").clicked() {
                    let mut i = 0;
                    query.iter().for_each(|entity| {
                        if i != 0 {
                            commands.entity(entity).despawn();
                        }
                        i += 1;
                    });
                    qaq::GLOBAL_STATE.lock().unwrap().children.clear();
                    init_render_document(ui_state, commands, asset_server)
                }
            });
        });
}

// Execute after the update phase.
// This phase can be used to deal with systems that need to run after the main update logic.
// fn post_update(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
// }

// Executes when the application is closed.
// This phase can be used to perform cleanup.
// fn last() {}
