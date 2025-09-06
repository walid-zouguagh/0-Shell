use std::io::{self, Write};

use crate::parser::parse_command;
use crate::commands;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&self) -> Result<(), String> {
        let stdin = io::stdin();
        // let mut stdout = io::stdout();

        loop {
            // Prompt
            print!("$ ");
            if let Err(e) =  io::stdout().flush() {
               eprintln!("{}",e);
               break;
            }

            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(0) => {
                    println!();
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("read error: {}", e);
                    continue;
                }
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            // Use parser
            let (cmd, args) = parse_command(input);
            if cmd.is_empty() {
                continue;
            }

            // Dispatch command
            let res = commands::dispatch(&cmd, &args);

            if let Err(msg) = res {
                eprintln!("{}", msg);
            }
        }
        Ok(())
    }
}
