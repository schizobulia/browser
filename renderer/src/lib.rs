mod component;
mod generate;

use bevy::prelude::*;
// use bevy_egui::EguiContexts;
use bean::node::{ElementText, Node};
use bean::ui_state::UiState;
use generate::NodeResult;
use js_engine;
use bean::qaq;
use scraper::{ElementRef, Html};
#[derive(Component)]
struct AnimateTranslation;

pub fn render_document(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _ui_state: ResMut<UiState>,
    html: String,
) {
    let mut js_runtime = js_engine::V8Runtime::new();
    js_runtime.init_global();
    let root = NodeBundle {
        style: Style {
            top: Val::Px(25.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: Color::rgba(255.0, 255.0, 255.0, 1.0).into(),
        ..default()
    };
    let mut sc = Vec::new();

    commands
        .spawn(root)
        .with_children(|parent: &mut ChildBuilder<'_>| {
            let mut binding = qaq::GLOBAL_STATE.lock().unwrap();
            let document = Html::parse_document(&html);
            traverse_html(
                document.root_element(), parent, &asset_server, 
                &mut js_runtime, binding.children.as_mut(), &mut sc);
        });
    for script in sc {
        let code = Box::leak(script.clone().into_boxed_str());
        js_runtime.eval(code);
    }
    // js.eval("const arr = [1, 2, 3];Deno.core.ops.op_sum(arr);");
}

fn traverse_html(
    element: ElementRef,
    commands: &mut ChildBuilder<'_>,
    asset_server: &Res<AssetServer>,
    js_runtime: &mut js_engine::V8Runtime,
    list: &mut Vec<Node>,
    sc: &mut Vec<String>,
) {
    let tag = element.value().name().to_string();
    let mut attributes: Vec<(String, String)> = Vec::new();
    element.value().attrs.clone().iter().for_each(|attr| {
        attributes.push((attr.0.local.to_string(), attr.1.to_string()));
    });
    let mut el_data: Node = Node {
        children: Vec::new(),
        tag_name: tag,
        attributes: attributes,
        text: None,
        id: None,
    };

    let res = generate::get_node_result(element);
    match res {
        // mark 大部分浏览器的逻辑是：在渲染过程中可以修改已经渲染好的dom，
        // 但目前这里的实现(GLOBAL_STATE)存在私锁的问题。
        NodeResult::Script(script) => {
            // let code = Box::leak(script.clone().into_boxed_str());
            // js_runtime.eval(code);
            sc.push(script);
        }
        NodeResult::Style(style) => {
            println!("{:?}", style);
        }
        NodeResult::Div(bundle) => {
            let id = commands
                .spawn(bundle)
                .with_children(|parent: &mut ChildBuilder<'_>| {
                    for child in element.children() {
                        if let Some(child_element) = ElementRef::wrap(child) {
                            traverse_html(child_element.clone(), parent, asset_server,
                                js_runtime, &mut el_data.children, sc);
                        } else if child.value().is_text() {
                            let text = child.value().as_text().unwrap().to_string();
                            let text_bundle = TextBundle::from_section(
                                &text,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                    color: Color::BLACK,
                                    ..default()
                                },
                            );
                            let childern_id = parent.spawn(text_bundle).id();
                            el_data.text = Some(ElementText {
                                id: Some(childern_id),
                                text,
                            });
                        }
                    }
                })
                .id();
            el_data.id = Some(id);
        }
    };
    list.push(el_data);
}

pub fn update_document(mut query: Query<&mut Text>
) {
    let mut binding_action = qaq::GLOBAL_ACTION.lock().unwrap();
    while binding_action.actions.len() > 0 {
        let ac = binding_action.actions.remove(0);
        match ac {
            qaq::Action::ChangeTextAction(change_text) => {
                let text = query.get_mut(change_text.id);
                match text {
                    Ok(mut t) => {
                        t.sections[0].value = change_text.value.clone();
                    },
                    Err(err) => {
                        println!("err: {:?}", err);
                    }
                }
            }
        }
    }
    // for mut text in &mut query {
    //     text.sections[0].value = ui_state.name.clone();
    // }
    // query.get_mut(entity)
}
