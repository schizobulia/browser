use scraper::{ElementRef, Html};

pub fn compile_html(html: &str) -> ElementRef {
    let document = Html::parse_document(html);
    // return document.root_element();
    // let root = traverse(document.root_element());
    // println!("{:#?}", root);
}

