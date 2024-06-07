mod js;

use bean::qaq;
use deno_core::*;

pub struct V8Runtime {
    runtime: JsRuntime,
}

impl V8Runtime {
    pub fn new() -> Self {
        const DECL: OpDecl = op_sum();
        const DOCUMENT_BY_ID: OpDecl = get_element_by_id();
        const CHANGE_ELEMENT_TEXT: OpDecl = change_element_text();
        let ext = Extension {
            name: "my_ext",
            ops: std::borrow::Cow::Borrowed(&[DECL, DOCUMENT_BY_ID, CHANGE_ELEMENT_TEXT]),
            ..Default::default()
        };

        let runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![ext],
            ..Default::default()
        });
        Self { runtime }
    }

    // Mark: should be set here to inject global variables into v8, which is only used temporarily
    pub fn init_global(&mut self) {
        self.eval(js::get_init_js_code());
    }

    pub fn eval(&mut self, code: &'static str) {
        let res = self.runtime.execute_script("<anon>", code);
        match res {
            Ok(global) => {
                let scope = &mut self.runtime.handle_scope();
                let _ = v8::Local::new(scope, global);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        };
    }
}

#[op2(async)]
async fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    Ok(sum)
}

/**
 * Get element by id
 * such as: document.getElementById('id')
 */
#[op2]
#[string]
fn get_element_by_id(#[string] id: String) -> Result<String, deno_core::error::AnyError> {
    match qaq::GLOBAL_STATE.lock() {
        Ok(node) => {
            let list = node.get_node_by_tag_id(id);
            match list {
                Some(node) => match node.lock() {
                    Ok(tmp) => Ok(tmp.to_json()),
                    Err(_) => Ok(String::new()),
                },
                None => Ok(String::new()),
            }
        }
        Err(_) => Ok(String::new()),
    }
}

/**
 * Change element text
 * such as: document.getElementById('id').innerText = 'new text'
 */
#[op2]
#[string]
fn change_element_text(
    #[string] id: String,
    #[string] value: String,
) -> Result<String, deno_core::error::AnyError> {
    match qaq::GLOBAL_STATE.lock() {
        Ok(node) => match id.parse::<u64>() {
            Ok(id_val) => {
                if let Some(data) = node.get_node_by_id(id_val) {
                    match data.lock() {
                        Ok(mut d) => match d.text {
                            Some(ref mut text) => match qaq::GLOBAL_ACTION.lock() {
                                Ok(mut binding_action) => {
                                    if let Some(i) = text.id {
                                        binding_action.actions.push(qaq::Action::ChangeTextAction(
                                            qaq::ChangeText {
                                                id: i,
                                                value: value.clone(),
                                            },
                                        ));
                                        text.text = value;
                                    }
                                }
                                Err(err) => {
                                    println!("{:?}", err);
                                }
                            },
                            None => {}
                        },
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        Err(err) => {
            println!("{:?}", err);
        }
    };
    Ok(String::new())
}
