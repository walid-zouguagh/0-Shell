use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Recursively copies a directory
fn copy_dir_recursively(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn cp(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cp: missing file operand".into());
    }

    let mut recursive = false;
    let mut paths: Vec<String> = Vec::new();

    // Handle flags (only -r for now)
    for arg in args {
        if arg == "-r" || arg == "-R" {
            recursive = true;
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.len() < 2 {
        return Err("cp: missing destination file operand".into());
    }

    let dest = Path::new(paths.last().unwrap());
    let sources = &paths[..paths.len() - 1];

    for src in sources {
        let src_path = Path::new(src);

        if !src_path.exists() {
            println!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }

        if src_path.is_dir() {
            if !recursive {
                println!("cp: -r not specified; omitting directory '{}'", src);
                continue;
            }

            let target = if dest.is_dir() {
                dest.join(src_path.file_name().unwrap())
            } else {
                dest.to_path_buf()
            };

            if let Err(e) = copy_dir_recursively(&src_path, &target) {
                println!("cp: failed to copy directory '{}': {}", src, e);
            }
        } else {
            let target = if dest.is_dir() {
                dest.join(src_path.file_name().unwrap())
            } else {
                dest.to_path_buf()
            };

            if let Err(e) = fs::copy(&src_path, &target) {
                println!("cp: failed to copy file '{}': {}", src, e);
            }
        }
    }

    Ok(())
}
