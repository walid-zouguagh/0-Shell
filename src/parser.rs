use std::io::{ self, Write };
pub fn parse_command(initial_input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;
    let mut input = initial_input.trim().to_string();
    
    // if hadak l inut didn't passe the function is_command_complet we gonna keep listening for the input !!!
    while !is_command_complete(&input) {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("{}", e);
        }

        // println!(" the problem is heeeer 09... ");
        let mut extra = String::new();
        if io::stdin().read_line(&mut extra).unwrap_or(0) == 0 {
            break;
        }
        input.push('\n');
        input.push_str(extra.trim_end());
    }

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '~' => {
                if current.is_empty() {
                    if let Ok(home) = std::env::var("HOME") {
                        current.push_str(&home);
                        // continue;
                    } else {
                        current.push('~');
                    }
                }
            }
            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                // in this we
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            // in case the arg starts with a # and is not in quotes we egnore the rest of the line
            '#' if !in_single_quotes && !in_double_quotes && current.is_empty() => {
                break;
            }
            // in case of a double quote we should check if we are not in single quotes then change the state of in_double_quotes
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            // here we do the same for single quotes
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            // in case of a backslash we set the escaped flag to true so that the next char is added as it is
            '\\' => {
                // let mut count = 1;
                // while let Some(&'\\') = chars.peek() {
                //     chars.next();
                //     count +=1;

                // }

                // Inside single quotes: literal backslash
                // current.push('\\');
                // println!("here1111111111111111111");
                //     continue;
                // } else {
                // println!("here2222222222222222222");

                // In double quotes or unquoted: check next char for escape
                // escaped = true;

                if in_single_quotes || in_double_quotes {
                    if let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            'n' => {chars.next();current.push('\n');}
                            't' => {chars.next();current.push('\t');}
                            'r' => {chars.next();current.push('\r');}
                            '\\' => {chars.next();current.push('\\');}
                            '"' if in_double_quotes => {chars.next();current.push('"');}
                            '\'' if !in_double_quotes => {chars.next();current.push('\'');}
                            // ' '
                                    
                            _ => {escaped =  true;}
                                // current.push('\\');
                            
                            //  No special escape, just add the backslash and the next char normally
                        } 
                    } else {
                        current.push('\\');
                    }
                } else {
                    escaped = true
                }
            }
            // continue;
            // }
            // } else {
            // println!("here33333333333333333");
            // Backslash at the end: treat as literal
            // }

            // the default state is to push the character to l arrg
            _ => {
                current.push(ch);
                // chars.next();
            }
        }
    }

    // in this case  the last char is a backslash we should add it to the current arg
    if escaped {
        // println!("here44444444444444444");
        current.push('\\');
    }

    if !current.is_empty() {
        args.push(current);
    }
    // in case of empty input....
    if args.is_empty() {
        return ("".to_string(), vec![]);
    }

    // here we remove the first element of the args and return it as the command
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
            '\\' => {
                escaped = true;
            }
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            _ => {}
        }
    }
    if in_single_quotes || in_double_quotes {
        return false;
    }
    let mut backslashees = 0;
    for ch in input.chars().rev() {
        if ch == '\\' {
            backslashees += 1;
        } else {
            break;
        }
    }
    backslashees % 2 == 0
    // !in_single_quotes && !in_double_quotes && !input.trim_end().ends_with('\\')
}
