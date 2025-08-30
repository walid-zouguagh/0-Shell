
pub fn clear(_args: &[String]) -> Result<(), String> {
    if _args.len() > 0 {
        return Err("clear: the command clear doesn't support options one is enough".into());
    }
    print!("\x1B[2J\x1B[H");
    Ok(())
}