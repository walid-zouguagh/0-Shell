use std::fs;
use std::path::Path;

pub fn cp(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cp: missing file operand".into());
    }
    // if paths.len() < 2 {
    //     return Err("cp: missing destination file operand".into());
    // }

    let binding = "".to_string();
    let dest = Path::new(args.last().unwrap_or(&binding));
    let source = &args[..args.len() - 1];
    if !dest.exists() {
        return Err(format!("cp: target '{}' does not exist", dest.display()));
    }

    for src in source {
        let src_path = Path::new(src);

        if !src_path.exists() {
            println!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }

        let target = if dest.is_dir() {
            if let Some(file_name) = src_path.file_name() {
                dest.join(file_name)
            } else {
                println!("cp: invalid source file '{}'", src);
                continue;
            }
        } else {
            // let target = if dest.is_dir() {
            //     dest.join(src_path.file_name().unwrap())
            // } else {
            //     dest.to_path_buf()
            dest.to_path_buf()
        };

        if let Err(e) = fs::copy(&src_path, &target) {
            println!("cp: failed to copy file '{}': {}", src, e);
        }
    }

    Ok(())
}
