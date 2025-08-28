// pub fn parse_command(input: &str) -> (String, Vec<String>) {
//     let mut parts = input.split_whitespace();
//     let cmd = parts.next().unwrap_or("").to_string();
//     let args: Vec<String> = parts.map(|s| s.to_string()).collect();
//     (cmd, args)
// }

pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                chars.next();
            }
            '#' if !in_quotes => {
                break;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
                chars.next();
            }
            _ => {
                current.push(ch);
                chars.next();
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
