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

//===================v1=====================//
// use std::fs;
// use std::os::unix::fs::MetadataExt;
// use chrono::{DateTime, Local};

// pub fn run(args: &[String]) -> Result<(), String> {
//     let entries: Vec<_> = fs::read_dir(".")

//         .map_err(|e| format!("ls: error reading directory: {}", e))?
//         .collect::<Result<_, _>>()
//         .map_err(|e| format!("ls: error reading entry: {}", e))?;

//     if args.is_empty() {
//         for entry in &entries {
//             println!("{}", entry.file_name().to_string_lossy());
//         }
//         return Ok(());
//     }

//     if args.len() == 1 && args[0] == "-l" {
//         for entry in &entries {
//             let meta = entry.metadata()
//                 .map_err(|e| format!("ls: error getting metadata: {}", e))?;

//              let file_type = if meta.is_dir() { 'd' } else { '-' };
//             let perms = meta.mode() & 0o777;
//             let perms_str = format!(
//                 "{}{}{}",
//                 perm_str((perms >> 6) & 7),
//                 perm_str((perms >> 3) & 7),
//                 perm_str(perms & 7)
//             );

//             let time: DateTime<Local> = meta.modified().unwrap().into();
//             let time_str = time.format("%b %e %H:%M").to_string();

//             println!(
//                 "{}{} {:>2} {:>5} {:>5} {:>8} {} {}",
//                 file_type,
//                 perms_str,
//                 meta.nlink(),
//                 meta.uid(),
//                 meta.gid(),
//                 meta.size(),
//                 time_str,
//                 entry.file_name().to_string_lossy()
//             );
//         }
//         return Ok(());
//     }

//     Err(format!("ls: invalid option '{}'", args[0]))
// }

// fn perm_str(bits: u32) -> String {
//     format!(
//         "{}{}{}",
//         if bits & 4 != 0 { "r" } else { "-" },
//         if bits & 2 != 0 { "w" } else { "-" },
//         if bits & 1 != 0 { "x" } else { "-" },
//     )
// }

//========================v2====================================

use chrono::{DateTime, Local};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt}; // for permissions, size, owner, group, etc
use std::time::SystemTime;

pub fn run(args: &[String]) -> Result<(), String> {
    // Flags
    let mut show_all = false; // -a
    let mut long_format = false; // -l
    let mut append_types = false; // -F

    // Paths
    let mut paths = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_all = true,
                    'l' => long_format = true,  // Handle the -l flag
                    'F' => append_types = true, // Handle the -F flag
                    _ => return Err(format!("ls: invalid option -- '{}'", ch)),
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.is_empty() {
        paths.push(
            env::current_dir()
                .map_err(|e| format!("ls: cannot get current dir: {}", e))?
                .to_string_lossy()
                .to_string(),
        );
    }

    for path_str in paths {
        let path = std::path::PathBuf::from(&path_str);

        // Check if the path exists
        if !path.exists() {
            println!(
                "ls: cannot access '{}': No such file or directory",
                path.display()
            );
            continue; // Skip to the next path
        }

        // If it's a directory, list its contents
        if path.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(&path)
                .map_err(|e| format!("ls: cannot access '{}': {}", path.display(), e))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("ls: error reading dir: {}", e))?;

            // Sort entries by file name
            entries.sort_by_key(|e| e.file_name());

            if long_format {
                // Long format requires more detailed output, so we print file details first
                print_long_format(&entries, show_all);
            } else {
                // Print all files in a horizontal line
                let mut first = true; // To avoid leading space
                for entry in entries {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    // Always show . and .. in -a flag
                    if !show_all
                        && file_name_str.starts_with('.')
                        && file_name_str != "."
                        && file_name_str != ".."
                    {
                        continue;
                    }

                    let file_name_str = if append_types {
                        append_type_suffix(&entry, file_name_str.to_string()) // No `?` here
                    } else {
                        file_name_str.into_owned()
                    };

                    // Print file names horizontally with space in between
                    if !first {
                        print!("   "); // Space between filenames
                    }
                    print!("{}", file_name_str);
                    first = false;
                }
                println!(); // Add final newline after all files
            }
        } else {
            // Handle case where path is not a directory, print it directly
            println!("{}", path.display());
        }
    }

    Ok(())
}

// Print the details for each file in long format (-l)
fn print_long_format(entries: &[fs::DirEntry], show_all: bool) {
    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Show hidden files if -a flag is set or for long format
        if !show_all
            && file_name_str.starts_with('.')
            && file_name_str != "."
            && file_name_str != ".."
        {
            continue;
        }

        // Print detailed information in long format
        let metadata = entry
            .metadata()
            .map_err(|e| format!("ls: metadata error: {}", e))
            .unwrap();
        let perms = format_mode(metadata.mode(), metadata.is_dir());
        let size = metadata.size();
        let links = metadata.nlink(); // Number of hard links
        let owner = get_owner(metadata.uid()).unwrap_or_else(|_| String::from("Unknown"));
        let group = get_group(metadata.gid()).unwrap_or_else(|_| String::from("Unknown"));

        let mtime = metadata.modified().unwrap_or(SystemTime::now());
        let datetime: DateTime<Local> = mtime.into();

        // Format like `Aug 30 12:00`
        let mtime_str = datetime.format("%b %e %H:%M").to_string();

        // Print all information in long format
        println!(
            "{} {:>5} {:>8} {:<8} {:<8} {} {}",
            perms,
            links,         // Hard links count
            owner,         // Owner
            group,         // Group
            size,          // Size
            mtime_str,     // Last modified
            file_name_str  // Filename with suffix
        );
    }
}

// Append special characters to file names based on their type (for `-F`)
fn append_type_suffix(entry: &fs::DirEntry, file_name: String) -> String {
    let file_type = match entry.file_type() {
        Ok(ft) => ft,
        Err(e) => {
            // Log the error and return the original file name if file_type fails
            eprintln!("Error getting file type: {}", e);
            return file_name; // Return the original file name if there's an error
        }
    };

    let mut file_name_str = file_name;

    // Append type-specific suffixes based on file type
    if file_type.is_dir() {
        file_name_str.push('/');
    } else if file_type.is_symlink() {
        file_name_str.push('@');
    } else if file_type.is_fifo() {
        file_name_str.push('|');
    } else if file_type.is_socket() {
        file_name_str.push('=');
    } else if file_type.is_file() && is_executable(&entry) {
        file_name_str.push('*');
    }

    file_name_str
}

// Check if a file is executable (for the `-F` flag)
fn is_executable(entry: &fs::DirEntry) -> bool {
    entry
        .metadata()
        .map(|meta| {
            meta.permissions().mode() & 0o100 != 0 // Check the execute bit
        })
        .unwrap_or(false)
}

// Convert mode bits into string like `-rw-r--r--` or `drwxr-xr-x`
fn format_mode(mode: u32, is_dir: bool) -> String {
    let file_type = if is_dir { 'd' } else { '-' };

    let mut perms = String::new();
    perms.push(file_type);

    let modes = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'),
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'),
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'),
    ];

    for (bit, ch) in modes {
        if mode & bit != 0 {
            perms.push(ch);
        } else {
            perms.push('-');
        }
    }

    perms
}

// Get the user (owner) name from UID
fn get_owner(uid: u32) -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("getent")
        .arg("passwd")
        .arg(uid.to_string())
        .output()
        .map_err(|e| format!("Failed to get owner: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to retrieve owner for UID {}", uid));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let name = output_str.split(':').next().unwrap_or_default();
    Ok(name.to_string())
}

// Get the group name from GID
fn get_group(gid: u32) -> Result<String, String> {
    use std::process::Command;

    let output = Command::new("getent")
        .arg("group")
        .arg(gid.to_string())
        .output()
        .map_err(|e| format!("Failed to get group: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to retrieve group for GID {}", gid));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let name = output_str.split(':').next().unwrap_or_default();
    Ok(name.to_string())
}
