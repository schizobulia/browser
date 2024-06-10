use std::{env, fs, io, path::Path};
fn main() {
    match env::current_dir() {
        Ok(path) => {
            let network_dir = path.join("assets").join("network");
            if !network_dir.exists() {
                match fs::create_dir(network_dir.clone()) {
                    Ok(_) => {
                        println!("The network directory was created successfully");
                    }
                    Err(e) => {
                        println!("The network directory could not be created: {}", e);
                    }
                }
            } else {
                match remove_dir_contents(network_dir.clone()) {
                    Ok(_) => {
                        println!("The network directory was cleaned successfully");
                    }
                    Err(e) => {
                        println!("The network directory could not be cleaned: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("The current directory can't be retrieved: {}", e);
        }
    }
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
