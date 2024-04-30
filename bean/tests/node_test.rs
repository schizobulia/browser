// use bean;
// use bean::node::{get_children_by_tag_name, get_node_by_id, Node};

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bevy::prelude::Entity;

//     #[test]
//     fn test_get_children_by_tag_name() {
//         let mut nodes = vec![
//             Node {
//                 children: vec![],
//                 tag_name: "div".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(1)),
//             },
//             Node {
//                 children: vec![],
//                 tag_name: "span".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(2)),
//             },
//             Node {
//                 children: vec![],
//                 tag_name: "div".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(3)),
//             },
//         ];

//         let result = get_children_by_tag_name("div", &mut nodes);
//         assert_eq!(result, vec![Entity::from_raw(1), Entity::from_raw(3)]);
//     }

//     #[test]
//     fn test_get_node_by_id() {
//         let mut nodes = vec![
//             Node {
//                 children: vec![],
//                 tag_name: "div".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(1)),
//             },
//             Node {
//                 children: vec![],
//                 tag_name: "span".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(2)),
//             },
//             Node {
//                 children: vec![],
//                 tag_name: "div".to_string(),
//                 attributes: vec![],
//                 text: None,
//                 id: Some(Entity::from_raw(3)),
//             },
//         ];

//         let node = get_node_by_id(&mut nodes, Entity::from_raw(2));
//         assert_eq!(node.unwrap().tag_name, "span");
//     }
// }
