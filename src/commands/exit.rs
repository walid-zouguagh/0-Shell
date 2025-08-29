pub fn run(args: &[String]) -> Result<(), String> {
    let code = args.get(0)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    std::process::exit(code);
}
