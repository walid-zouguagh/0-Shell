// use std::fs;

// pub fn run(args: &[String]) -> Result<(), String> {
//     if !args.is_empty() {
//         return Err("ls: No such file or directory (os error 2)".to_string());
//     }

//     let entries = fs::read_dir(".")
//         .map_err(|e| format!("ls: error reading directory: {}", e))?;

//     for entry in entries {
//         let entry = entry.map_err(|e| format!("ls: error reading entry: {}", e))?;
//         let file_name = entry.file_name();
//         println!("{}", file_name.to_string_lossy());
//     }

//     Ok(())
// }

use std::fs;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Local};

pub fn run(args: &[String]) -> Result<(), String> {
    let entries: Vec<_> = fs::read_dir(".")

        .map_err(|e| format!("ls: error reading directory: {}", e))?
        .collect::<Result<_, _>>()
        .map_err(|e| format!("ls: error reading entry: {}", e))?;

    if args.is_empty() {
        for entry in &entries {
            println!("{}", entry.file_name().to_string_lossy());
        }
        return Ok(());
    }

    if args.len() == 1 && args[0] == "-l" {
        for entry in &entries {
            let meta = entry.metadata()
                .map_err(|e| format!("ls: error getting metadata: {}", e))?;

             let file_type = if meta.is_dir() { 'd' } else { '-' };
            let perms = meta.mode() & 0o777;
            let perms_str = format!(
                "{}{}{}",
                perm_str((perms >> 6) & 7),
                perm_str((perms >> 3) & 7),
                perm_str(perms & 7)
            );

            let time: DateTime<Local> = meta.modified().unwrap().into();
            let time_str = time.format("%b %e %H:%M").to_string();

            println!(
                "{}{} {:>2} {:>5} {:>5} {:>8} {} {}",
                file_type,
                perms_str,
                meta.nlink(),
                meta.uid(),
                meta.gid(),
                meta.size(),
                time_str,
                entry.file_name().to_string_lossy()
            );
        }
        return Ok(());
    }
    
    Err(format!("ls: invalid option '{}'", args[0]))
}

fn perm_str(bits: u32) -> String {
    format!(
        "{}{}{}",
        if bits & 4 != 0 { "r" } else { "-" },
        if bits & 2 != 0 { "w" } else { "-" },
        if bits & 1 != 0 { "x" } else { "-" },
    )
}
