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
                    'l' => long_format = true,
                    'F' => append_types = true,
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
            let entries: Vec<_> = fs::read_dir(&path)
                .map_err(|e| format!("ls: cannot access '{}': {}", path.display(), e))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("ls: error reading dir: {}", e))?;

            if long_format {
                print_long_format(&entries, show_all, &path, append_types)?;
            } else {
                print_short_format(&entries, show_all, append_types)?;
            }
        } else {
            // Handle case where path is a file, not a directory
            if long_format {
                let metadata = fs::metadata(&path)
                    .map_err(|e| format!("ls: cannot access '{}': {}", path.display(), e))?;
                let file_name = path
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new(""))
                    .to_string_lossy()
                    .to_string();
                print_long_entry(&metadata, &file_name, append_types)?;
            } else {
                let mut file_name = path.display().to_string();
                if append_types {
                    file_name = append_type_suffix_for_path(&path, file_name)?;
                }
                println!("{}", file_name);
            }
        }
    }

    Ok(())
}

// Print files in short format (default)
fn print_short_format(
    entries: &[fs::DirEntry],
    show_all: bool,
    append_types: bool,
) -> Result<(), String> {
    // Create a list of all entries to display
    let mut display_entries = Vec::new();

    // Add . and .. if -a flag is set (these will be sorted with everything else)
    if show_all {
        display_entries.push(".".to_string());
        display_entries.push("..".to_string());
    }

    // Add regular entries
    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files unless -a is given
        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        let file_name_str = if append_types {
            append_type_suffix(entry, file_name_str.to_string())?
        } else {
            file_name_str.into_owned()
        };

        display_entries.push(file_name_str);
    }

    // Sort all entries together using ls-like sorting:
    // 1. . comes first
    // 2. .. comes second
    // 3. Everything else sorted case-insensitively, ignoring leading dots for sort order
    display_entries.sort_by(|a, b| {
        match (a.as_str(), b.as_str()) {
            (".", _) => std::cmp::Ordering::Less,
            (_, ".") => std::cmp::Ordering::Greater,
            ("..", _) if b != "." => std::cmp::Ordering::Less,
            (_, "..") if a != "." => std::cmp::Ordering::Greater,
            _ => {
                // For other files, sort by name ignoring leading dots and case
                let key_a = a.trim_start_matches('.').to_lowercase();
                let key_b = b.trim_start_matches('.').to_lowercase();
                key_a.cmp(&key_b)
            }
        }
    });

    // Print all entries
    let mut first = true;
    for entry in display_entries {
        if !first {
            print!("   ");
        }
        print!("{}", entry);
        first = false;
    }

    if !first {
        println!(); // Add final newline only if we printed something
    }

    Ok(())
}

// Print the details for each file in long format (-l)
fn print_long_format(
    entries: &[fs::DirEntry],
    show_all: bool,
    path: &std::path::Path,
    append_types: bool,
) -> Result<(), String> {
    let mut total_blocks = 0;
    let mut display_entries = Vec::new();

    // Add . and .. if -a flag is set
    if show_all {
        if let Ok(meta) = fs::metadata(path) {
            total_blocks += meta.blocks();
            display_entries.push((meta, ".".to_string()));
        }
        if let Some(parent) = path.parent() {
            if let Ok(meta) = fs::metadata(parent) {
                total_blocks += meta.blocks();
                display_entries.push((meta, "..".to_string()));
            }
        }
    }

    // Add regular entries
    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = entry
            .metadata()
            .map_err(|e| format!("ls: metadata error: {}", e))?;

        total_blocks += metadata.blocks();

        let file_name_str = if append_types {
            append_type_suffix(entry, file_name_str.to_string())?
        } else {
            file_name_str.into_owned()
        };

        display_entries.push((metadata, file_name_str));
    }

    // Sort entries using the same logic as short format
    display_entries.sort_by(|a, b| match (a.1.as_str(), b.1.as_str()) {
        (".", _) => std::cmp::Ordering::Less,
        (_, ".") => std::cmp::Ordering::Greater,
        ("..", _) if b.1 != "." => std::cmp::Ordering::Less,
        (_, "..") if a.1 != "." => std::cmp::Ordering::Greater,
        _ => {
            let key_a = a.1.trim_start_matches('.').to_lowercase();
            let key_b = b.1.trim_start_matches('.').to_lowercase();
            key_a.cmp(&key_b)
        }
    });

    // GNU `ls -l` reports 1K blocks, while `metadata.blocks()` is 512-byte blocks.
    // Divide by 2 to match GNU `ls` behavior.
    println!("total {}", total_blocks / 2);

    // Print detailed info for all entries
    for (metadata, file_name) in display_entries {
        print_long_entry(&metadata, &file_name, append_types)?;
    }

    Ok(())
}

// Print detailed info for one file (used in -l)
fn print_long_entry(
    metadata: &fs::Metadata,
    file_name: &str,
    _append_types: bool,
) -> Result<(), String> {
    let perms = format_mode(metadata.mode(), metadata.is_dir());
    let size = metadata.size();
    let links = metadata.nlink(); // Number of hard links
    let owner = get_owner(metadata.uid()).unwrap_or_else(|_| String::from("unknown"));
    let group = get_group(metadata.gid()).unwrap_or_else(|_| String::from("unknown"));

    let mtime = metadata.modified().unwrap_or(SystemTime::now());
    let datetime: DateTime<Local> = mtime.into();

    // Format like `Aug 30 12:00`
    let mtime_str = datetime.format("%b %e %H:%M").to_string();

    // Print all information in long format
    println!(
        "{} {:>3} {:>8} {:>8} {:>8} {} {}",
        perms,
        links,     // Hard links count
        owner,     // Owner
        group,     // Group
        size,      // Size
        mtime_str, // Last modified
        file_name  // Filename with suffix
    );

    Ok(())
}

// Append special characters to file names based on their type (for `-F`)
fn append_type_suffix(entry: &fs::DirEntry, file_name: String) -> Result<String, String> {
    let file_type = entry
        .file_type()
        .map_err(|e| format!("Error getting file type: {}", e))?;

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
    } else if file_type.is_file() && is_executable(entry)? {
        file_name_str.push('*');
    }

    Ok(file_name_str)
}

// Append type suffix for a path (used when path is a file, not from DirEntry)
fn append_type_suffix_for_path(
    path: &std::path::Path,
    file_name: String,
) -> Result<String, String> {
    let metadata = fs::metadata(path).map_err(|e| format!("Error getting metadata: {}", e))?;

    let mut file_name_str = file_name;

    if metadata.is_dir() {
        file_name_str.push('/');
    } else if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        file_name_str.push('*');
    }
    // Note: For symlinks and other special files, we'd need to use fs::symlink_metadata()
    // and handle them appropriately, but this covers the basic cases

    Ok(file_name_str)
}

// Check if a file is executable (for the `-F` flag)
fn is_executable(entry: &fs::DirEntry) -> Result<bool, String> {
    let metadata = entry
        .metadata()
        .map_err(|e| format!("Error getting metadata: {}", e))?;

    Ok(metadata.permissions().mode() & 0o111 != 0)
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
        return Ok(uid.to_string()); // Return UID as fallback
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    if let Some(name) = output_str.split(':').next() {
        Ok(name.to_string())
    } else {
        Ok(uid.to_string())
    }
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
        return Ok(gid.to_string()); // Return GID as fallback
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    if let Some(name) = output_str.split(':').next() {
        Ok(name.to_string())
    } else {
        Ok(gid.to_string())
    }
}
