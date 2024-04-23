use bevy::{prelude::Entity};

#[derive(Debug, Clone)]
pub enum Node {
    Element(ElementData, Vec<Node>),
}

#[derive(Debug, Clone)]
pub struct ElementData {
    pub id: Option<Entity>,
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub text: Option<ElementText>,
}


#[derive(Debug, Clone)]
pub struct ElementText {
    pub id: Option<Entity>,
    pub text: String,
}

pub fn print_node(node: &Node) {
    match node {
        Node::Element(data, children) => {
            println!("tag_name: {:?}", data.tag_name);
            match &data.text {
                Some(text) => {
                    println!("text: {:?}", text.text);
                },
                None => {}
            }
            for child in children {
                print_node(child);
            }
        },
    }
}

/*
 * 根据tag_name修改子节点的text
 */
// pub fn change_text_by_tag_name(tag_name: &str, new_text: &str, list: &mut Vec<Node>) {
//     for node in list {
//         match node {
//             Node::Element(data, children) => {
//                 if data.tag_name == tag_name {
//                     // 如果当前节点的标签名与给定的标签名匹配，我们需要修改其所有文本子节点的内容
//                     for child in children.iter_mut() {
//                         if let Node::Text(text) = child {
//                             text.text = new_text.to_string();
//                         }
//                     }
//                 }
//                 // 无论当前节点的标签名是否匹配，我们都需要递归地处理其子节点
//                 change_text_by_tag_name(tag_name, new_text, children);
//             },
//             // 对于文本节点，我们不需要做任何事情
//             Node::Text(_) => {}
//         }
//     }
// }


/**
 * 根据tag_name获取子节点，获取到的子节点可以被修改
 */
pub fn get_children_by_tag_name<'a>(tag_name: &str, list: &'a mut Vec<Node>) -> Vec<&'a mut ElementData> {
    let mut result: Vec<&'a mut ElementData> = Vec::new();
    let mut queue = list.iter_mut().collect::<Vec<_>>();

    while !queue.is_empty() {
        let node = queue.remove(0);
        match node {
            Node::Element(data, children) => {
                if data.tag_name == tag_name {
                    queue.extend(children.iter_mut().collect::<Vec<_>>());
                    result.push(data);
                } else {
                    queue.extend(children.iter_mut());
                }
            }
        }
    }

    result
}
