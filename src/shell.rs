use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::commands;
use crate::parser::parse_command;

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }

    pub fn run(&self) -> Result<(), String> {
        let stdin = io::stdin();
        // let mut stdout = io::stdout();

        loop {
            // this condition is set to insure the the curent path is valiid else if weee gonna proviide the iuser
            // with the home directory so it woont panic orr give an errooor !!
            // not to be toucheeeed by anyone pleaseee !!!!
            if std::env::current_dir().is_err() {
                let backup_path = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
                if let Err(e) = std::env::set_current_dir(&backup_path) {
                    eprintln!("Failed to recover working directory: {}", e);

                    std::env::set_current_dir("/").ok();
                }
            }

            let pwd = match env::current_dir() {
                Ok(path) => path,
                Err(_e) => PathBuf::new(),
            };
            // Prompt
            print!("{}$ ", pwd.display());
            if let Err(e) = io::stdout().flush() {
                eprintln!("{}", e);
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
