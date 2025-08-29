use std::env;

pub fn run(_args: &[String]) -> Result<(), String> {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        }
        Err(e) => Err(format!("pwd: error: {}", e)),
    }
}

