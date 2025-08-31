pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut its_a_quote = false;
    let mut chars = input.trim().chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' if !in_quotes && ! its_a_quote => {
                if !current.is_empty(){
                    args.push(current.clone());
                    current.clear();
                }
            }
            '#' if !in_quotes => {
                break; // treat '#' as comment starter
            }
            '\"' if !in_quotes => {
                in_quotes =  !in_quotes
            }
            '\'' => {
                its_a_quote = !its_a_quote
            }
            '\\' => {
                if let Some(next_character) = chars.next(){
                    current.push(next_character)
                }else {
                    current.push(ch)
                }
            }
            _ => {
                current.push(ch);
                // chars.next();
            }
        }
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
