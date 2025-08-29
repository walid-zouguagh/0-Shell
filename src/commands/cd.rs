use std::env;

pub fn run(args: &[String]) -> Result<(), String> {
    let target = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        args[0].clone()
    };

    if let Err(e) = env::set_current_dir(&target) {
        return Err(format!("cd: {}: {}", target, e));
    }

    Ok(())
}