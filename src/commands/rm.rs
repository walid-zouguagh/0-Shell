use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("rm: missing operand".into());
    }

    for target in args {
        let path = Path::new(target);
        if !path.exists() {
            println!("rm: cannot remove '{}': No such file or directory", target);
            continue;
        }
    // now we are sure the path is valiiiid 
    // soo we need to check if its a directory or a file
        if path.is_dir() {
            println!("rm: cannot remove '{}': Is a directory", target);
            continue;
        }
    // daba we can ramove l file bla mashakiil 
        if let Err(e) = fs::remove_file(path) {
            println!("rm: failed to remove '{}': {}", target, e);
        }
    }

    Ok(())
}
