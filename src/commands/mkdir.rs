use std::fs;
use std::path::Path;

pub fn mkdir(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".into());
    }

    for dir in args {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir(path) {
            return Err(format!("mkdir: cannot create directory '{}': {}", dir, e));
        }
    }

    Ok(())
}
