use bean;
use bean::node::{get_children_by_tag_name, ElementData, ElementText, Node};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_children_by_tag_name() {
        let node = Node::Element(
            ElementData {
                id: None,
                tag_name: "html".to_string(),
                attributes: Vec::new(),
                text: None,
            },
            vec![Node::Element(
                ElementData {
                    id: None,
                    tag_name: "body".to_string(),
                    attributes: Vec::new(),
                    text: None,
                },
                vec![
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "p".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "1".to_string(),
                            }),
                        },
                        Vec::new(),
                    ),
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "p".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "2".to_string(),
                            }),
                        },
                        Vec::new(),
                    ),
                ],
            )],
        );
        assert!(get_children_by_tag_name("p", &mut vec![node]).len() == 2);
    }

    #[test]
    fn test_get_children_by_tag_name_2() {
        let node = Node::Element(
            ElementData {
                id: None,
                tag_name: "html".to_string(),
                attributes: Vec::new(),
                text: None,
            },
            vec![Node::Element(
                ElementData {
                    id: None,
                    tag_name: "body".to_string(),
                    attributes: Vec::new(),
                    text: None,
                },
                vec![
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "1".to_string(),
                            }),
                        },
                        Vec::new(),
                    ),
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "2".to_string(),
                            }),
                        },
                        Vec::new(),
                    ),
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "p".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "3".to_string(),
                            }),
                        },
                        Vec::new(),
                    ),
                ],
            )],
        );
        assert!(get_children_by_tag_name("div", &mut vec![node]).len() == 2);
    }

    #[test]
    fn test_get_children_by_tag_name_3() {
        let node = Node::Element(
            ElementData {
                id: None,
                tag_name: "html".to_string(),
                attributes: Vec::new(),
                text: None,
            },
            vec![Node::Element(
                ElementData {
                    id: None,
                    tag_name: "body".to_string(),
                    attributes: Vec::new(),
                    text: None,
                },
                vec![
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "1".to_string(),
                            }),
                        },
                        vec![
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "p".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "2".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "span".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "3".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                        ],
                    ),
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "4".to_string(),
                            }),
                        },
                        vec![
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "p".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "5".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "div".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "6".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                        ],
                    ),
                ],
            )],
        );
        assert!(get_children_by_tag_name("span", &mut vec![node]).len() == 1);
    }

    #[test]
    fn test_modify_text_by_tag_name() {
        let node = Node::Element(
            ElementData {
                id: None,
                tag_name: "html".to_string(),
                attributes: Vec::new(),
                text: None,
            },
            vec![Node::Element(
                ElementData {
                    id: None,
                    tag_name: "body".to_string(),
                    attributes: Vec::new(),
                    text: None,
                },
                vec![
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "Random Text 1".to_string(),
                            }),
                        },
                        vec![
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "p".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "Random Text 2".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "span".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "Random Text 3".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                        ],
                    ),
                    Node::Element(
                        ElementData {
                            id: None,
                            tag_name: "div".to_string(),
                            attributes: Vec::new(),
                            text: Some(ElementText {
                                id: None,
                                text: "Random Text 4".to_string(),
                            }),
                        },
                        vec![
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "p".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "Random Text 5".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                            Node::Element(
                                ElementData {
                                    id: None,
                                    tag_name: "span".to_string(),
                                    attributes: Vec::new(),
                                    text: Some(ElementText {
                                        id: None,
                                        text: "Random Text 6".to_string(),
                                    }),
                                },
                                Vec::new(),
                            ),
                        ],
                    ),
                ],
            )],
        );
        get_children_by_tag_name("div", &mut vec![node]).iter_mut().for_each(|a| {
            match &mut a.text {
                Some(text) => {
                    text.text = "Modified Text".to_string();
                    assert!(&a.text.as_ref().unwrap().text == "Modified Text");
                },
                None => {}
            }
        });
    }
}
