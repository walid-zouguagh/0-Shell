use std::io::{self, Write};

pub fn parse_command(initial_input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;
    let mut input = initial_input.trim().to_string();

    while !is_command_complete(&input) {
        print!("> "); // continuation prompt
        if let Err(e) = io::stdout().flush() {
            eprintln!("{}", e);
        }

        let mut extra = String::new();
        if io::stdin().read_line(&mut extra).unwrap() == 0 {
            break; // EOF
        }
        input.push('\n');
        input.push_str(extra.trim_end());
    }

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if escaped {
            if in_double_quotes || in_single_quotes {
                match ch {
                    'n' => current.push('\n'),
                    't' => current.push('\t'),
                    'r' => current.push('\r'),
                    '\\' => current.push('\\'),
                    '\'' => current.push('\''),
                    '"' => current.push('"'),
                    _ => {
                        current.push('\\');
                        current.push(ch);
                    }
                }
            } else {
                
                current.push('\\');
                current.push(ch);
            }
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
               
                if in_double_quotes || in_single_quotes {
                    escaped = true;
                } else {
                    current.push('\\');
                }
            }
            '~' => {
                if current.is_empty() && !in_single_quotes && !in_double_quotes {
                    if let Ok(home) = std::env::var("HOME") {
                        current.push_str(&home);
                    } else {
                        current.push('~');
                    }
                } else {
                    current.push('~');
                }
            }
            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            '#' if !in_single_quotes && !in_double_quotes && current.is_empty() => {
                break;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if escaped {
        current.push('\\');
    }

    if !current.is_empty() {
        args.push(current);
    }

    if args.is_empty() {
        return ("".to_string(), vec![]);
    }

    let cmd = args.remove(0);
    (cmd, args)
}

fn is_command_complete(input: &str) -> bool {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
            '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
            _ => {}
        }
    }

    !in_single_quotes && !in_double_quotes && !escaped
}