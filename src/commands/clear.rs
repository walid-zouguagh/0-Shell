
pub fn clear(_args: &[String]) -> Result<(), String> {
    if _args.len() > 0 {
        return Err("clear: the command clear doesn't support options one is enough".into());
    }
 
    //  this code is devided into two parts
    print!("\x1B[2J\x1B[H");
    // \x1B is the ecape character so the next part is treated as a command
    // 2J  i use this command to clear the screen
    // H   i use this command to move the cursor to the top-left corner
    Ok(())
}