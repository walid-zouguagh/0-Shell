use std::io::{self, Write};

use crate::parser::split_args;
use crate::commands;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&self) -> Result<(), String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Prompt
            print!("$ ");
            let _ = stdout.flush();

            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(0) => {
                    // EOF (Ctrl+D)
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

            let mut args = split_args(input);
            if args.is_empty() {
                continue;
            }

            let cmd = args.remove(0);
            let res = commands::dispatch(&cmd, &args);

            if let Err(msg) = res {
                eprintln!("{}", msg);
            }
        }
        Ok(())
    }
}
