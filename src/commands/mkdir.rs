use std::fs;
use std::path::Path;

pub fn mkdir(args: &[String]) -> Result<(), String> {
    // println!("{:?}", args);
    if args.is_empty() {
        return Err("mkdir: missing operand".into());
    }
    
    for dir in args {
        let path = Path::new(dir);
        if let Err(e) = fs::create_dir(path) {
            println!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
    
    Ok(())
}
