pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escaped = false;
    let mut chars = input.trim().chars().peekable();

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
                        continue;
                    }
                }
                current.push('~');
            }
            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                // in this we
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            // in case the arg starts with a # and is not in quotes we egnore the rest of the line
            '#' if !in_single_quotes && !in_double_quotes => {
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
                if !in_single_quotes {
                    // Inside single quotes: literal backslash
                    current.push('\\');
                } else {
                    // In double quotes or unquoted: check next char for escape
                    if let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            'n' => {
                                chars.next();
                                current.push('\n');
                            }
                            't' => {
                                chars.next();
                                current.push('\t');
                            }
                            _ => {
                                //  No special escape, just add the backslash and the next char normally
                                //  current.push('\\');
                                continue;
                            }
                        }
                    } else {
                        // Backslash at the end: treat as literal
                        current.push('\\');
                    }
                }
            }
            // '\n'  =>  {
            //     current.push(' ');
            // }
            // the default state is to push the character the the arrg
            _ => {
                current.push(ch);
                // chars.next();
            }
        }
    }
    // this condition here is to chech if our arg or command is between quotes else if its gonna return an err
    if in_single_quotes || in_double_quotes {
        return (" error: help : close the damn quote please".to_string(), vec![]);
    }
    // in this case  the last char is a backslash we should add it to the current arg
    if escaped {
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
