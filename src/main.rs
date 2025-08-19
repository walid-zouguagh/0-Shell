mod commands;
mod parser;
mod shell;
mod utils;

fn main() {
    if let Err(e) = shell::Shell::new().run(){
        eprintln!("Fatal Error: {}", e);
    }
}
