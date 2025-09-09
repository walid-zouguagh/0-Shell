mod commands;
mod parser;
mod shell;

fn main() {
    if let Err(e) = shell::Shell::new().run(){
        eprintln!("Fatal Error: {}", e);
    }
}
