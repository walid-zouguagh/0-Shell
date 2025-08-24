mod parser;
mod commands {
    pub mod echo;
    pub mod cd;
    pub mod pwd;
    pub mod ls;
    pub mod exit;
}

use std::io::{self, Write, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("$ ");
        stdout.flush().unwrap();

        let mut input = String::new();
        let bytes_read = stdin.lock().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let (cmd, args) = parser::parse_command(input);

        match cmd.as_str() {
            "echo" => commands::echo::run(&args),
            "cd" => commands::cd::run(&args),
            "pwd" => commands::pwd::run(),
            "ls" => commands::ls::run(&args),
            "exit" => {
            if commands::exit::run() {
                    break;
                }
          }

            _ => eprintln!("Command '{}' not found", cmd),
        }
    }
}
