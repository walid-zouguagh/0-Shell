pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut parts = input.split_whitespace();
    let cmd = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    (cmd, args)
}
