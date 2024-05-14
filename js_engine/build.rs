use std::{env, fs};

fn main() {
    match env::current_dir() {
        Ok(path) => match fs::read_to_string(path.join("script").join("index.js")) {
            Ok(content) => {
                write_to_file(path, content);
            }
            Err(e) => {
                println!("The file could not be read: {}", e);
            }
        },
        Err(e) => {
            println!("The current directory can't be retrieved: {}", e);
        }
    }
}

fn write_to_file(path: std::path::PathBuf, content: String) {
    let file_path = path.join("src").join("js.rs");
    let mut data = String::new();
    data.push_str("pub fn get_init_js_code() -> &'static str {");

    data.push_str("return \"");
    data.push_str(content.as_str());
    data.push_str("\"; }");

    match fs::write(file_path, data) {
        Ok(_) => {
            println!("The js file was written successfully");
        }
        Err(e) => {
            println!("The file could not be written: {}", e);
        }
    }
}
