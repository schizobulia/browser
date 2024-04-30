use bean::{qaq, ui_state::UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use network::get_html_by_url;
use renderer::{render_document, update_node_text};

pub fn open_window() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, start_up)
        // .add_systems(PreUpdate, pre_update)
        .add_systems(Update, (update, update_node_text))
        // .add_systems(PostUpdate, post_update)
        // .add_systems(Last, last)
        .run();
}

// 应用程序启动时执行。这个阶段通常用于初始化资源、设置状态和配置。
fn start_up(commands: Commands, asset_server: Res<AssetServer>, ui_state: ResMut<UiState>) {
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

// 更新阶段之前执行。这个阶段可以用于处理需要在主更新逻辑之前运行的系统。
// fn pre_update(mut commands: Commands, asset_server: Res<AssetServer>) {}

// 每帧更新时执行。这是执行逻辑的主要阶段。
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

// 更新阶段之后执行。这个阶段可以用于处理需要在主更新逻辑之后运行的系统。
// fn post_update(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
// }

// 应用程序关闭时执行。这个阶段可以用于执行清理工作。
// fn last() {}
