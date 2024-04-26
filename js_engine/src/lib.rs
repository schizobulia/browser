use bean::node::{get_node_by_tag_id, get_node_by_id, Node};
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

    // mark 这里应该设置为给v8注入全局变量，这里只是临时使用
    pub fn init_global(&mut self) {
        self.eval("    class Element {
            constructor(tag, innerText, _id) {
                this.tag = tag;
                this._id = _id
                this._text = innerText
                const descriptor = Object.getOwnPropertyDescriptor(Element.prototype, 'innerText');
                Object.defineProperty(this, 'innerText', {
                    get: () => { return this._text },
                    set: (value) => {
                        Deno.core.ops.change_element_text(_id.toString(), value);
                        this._text = value;
                    }
                });
            }
        }
        globalThis.Element = Element
        globalThis.document = {
            getElementById: (id) => {
                let data = Deno.core.ops.get_element_by_id(id);
                if (data) {
                   data = JSON.parse(data)
                   const res = new globalThis.Element(data.tag_name, '', data.id)
                   if (data.text) {
                        res._text = data.text.text
                   }
                   return res
                }
                return undefined
            }
        }");
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
    // let node = get_children_by_tag_name("p", qaq::GLOBAL_STATE.lock().unwrap().children.as_mut());
    Ok(sum)
}


#[op2]
#[string]
fn get_element_by_id(#[string] id: String) -> Result<String, deno_core::error::AnyError> {
    let mut binding = qaq::GLOBAL_STATE.lock().unwrap();
    let list: Option<&mut Node> = get_node_by_tag_id(id, binding.children.as_mut());
    let mut res: String = String::new();
    match list {
        Some(node) => {
            res = serde_json::to_string(&node).unwrap();
        },
        None => {
        }
    }
    Ok(res)
}

#[op2]
#[string]
fn change_element_text(#[string] id: String, #[string] value: String) -> Result<String, deno_core::error::AnyError> {
    let mut binding = qaq::GLOBAL_STATE.lock().unwrap();
    let data: Option<&mut Node> = get_node_by_id(binding.children.as_mut(), id.parse().unwrap());
    match data {
        Some(node) => {
            match node.text {
                Some(ref mut text) => {
                    let mut binding_action = qaq::GLOBAL_ACTION.lock().unwrap();
                    binding_action.actions.push(qaq::Action::ChangeTextAction(qaq::ChangeText {
                        id: text.id.unwrap(),
                        value: value.clone(),
                    }));
                    text.text = value;
                },
                None => {}
            }
        },
        None => {
        }
    }
    Ok(String::new())
}