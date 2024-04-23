use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use renderer::{render_document, update_document};
use bean::ui_state::UiState;

pub fn open_window() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, start_up)
        // .add_systems(PreUpdate, pre_update)
        .add_systems(Update, (update, update_document))
        // .add_systems(PostUpdate, post_update)
        // .add_systems(Last, last)
        .run();
}

// 应用程序启动时执行。这个阶段通常用于初始化资源、设置状态和配置。
fn start_up(mut commands: Commands, asset_server: Res<AssetServer>, ui_state: ResMut<UiState>) {
    // Camera
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
    render_document(commands, asset_server, ui_state);
}

// 更新阶段之前执行。这个阶段可以用于处理需要在主更新逻辑之前运行的系统。
// fn pre_update(mut commands: Commands, asset_server: Res<AssetServer>) {}

// 每帧更新时执行。这是执行逻辑的主要阶段。
fn update(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("Top panel")
        .exact_height(15.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| ui.text_edit_singleline(&mut ui_state.name));
        });
}


// 更新阶段之后执行。这个阶段可以用于处理需要在主更新逻辑之后运行的系统。
// fn post_update(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
// }

// 应用程序关闭时执行。这个阶段可以用于执行清理工作。
// fn last() {}
