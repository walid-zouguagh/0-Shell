use std::fs;
use std::path::Path;

pub fn cp(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cp: missing file operand".into());
    }

    let dest = Path::new(&args[args.len() - 1]);
    let sources = &args[..args.len() - 1];

    // If multiple sources, destination must be a directory
    if sources.len() > 1 && !dest.is_dir() {
        return Err(format!("cp: target '{}' is not a directory", dest.display()));
    }

    for src in sources {
        let src_path = Path::new(src);

        if !src_path.exists() {
            println!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }

        let dest_path = if dest.is_dir() {
            dest.join(src_path.file_name().unwrap())
        } else {
            dest.to_path_buf()
        };

        if let Err(e) = fs::copy(src_path, &dest_path) {
            println!("cp: cannot copy '{}' to '{}': {}", src, dest_path.display(), e);
        }
    }

    Ok(())
}
