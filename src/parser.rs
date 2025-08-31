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
            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty(){
                    args.push(current.clone());
                    current.clear();
                }
            }
            '#' if !in_single_quotes && !in_double_quotes => {
                break;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes
            }
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes
            }
            '\\' => {
               escaped = true
            }
            _ => {
                current.push(ch);
                // chars.next();
            }
        }
    }
     if in_single_quotes || in_double_quotes {
        return (" error: help : close the damn quote please".to_string(), vec![]);
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
