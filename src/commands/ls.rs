use chrono::{DateTime, Datelike, Local};
use libc::{major, minor};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::time::SystemTime;
use term_size;

pub fn run(args: &[String]) -> Result<(), String> {
    // Flags
    let mut show_all = false; // -a
    let mut long_format = false; // -l
    let mut append_types = false; // -F

    // Paths
    let mut paths = Vec::new();

    for arg in args {
        if arg == "-" {
            // Special case: "-" is not an option, it's a filename
            paths.push(arg.clone());
        } else if arg.starts_with('-') {
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

    // when you run ls without giving it any paths.
    if paths.is_empty() {
        paths.push(
            env::current_dir()
                .map_err(|e| format!("ls: cannot get current dir: {}", e))?
                .to_string_lossy()
                .to_string(),
        );
    }

    for path_str in paths {
        // let path = std::path::PathBuf::from(&path_str);
        let path = expand_path(&path_str);

        // Check if the path exists
        if !path.exists() {
            println!(
                "ls: cannot access '{}': No such file or directory",
                path.display()
            );
            continue; // Skip to the next path
        }

        // Use symlink_metadata to decide what the path really is
        let meta = fs::symlink_metadata(&path)
            .map_err(|e| format!("ls: cannot access '{}': {}", path.display(), e))?;

        // For long format, always show the file/symlink info itself
        // For short format, follow symlinks to directories
        let is_directory = if long_format {
            // In long format, don't follow symlinks - show the symlink itself
            meta.is_dir() && !meta.file_type().is_symlink()
        } else {
            // In short format, follow symlinks to directories
            if meta.file_type().is_symlink() {
                match fs::metadata(&path) {
                    Ok(target_meta) => target_meta.is_dir(),
                    Err(_) => false, // Broken symlink
                }
            } else {
                meta.is_dir()
            }
        };

        if is_directory {
            // Directory or (in short format) symlink pointing to a directory - list contents
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
            // Regular file, symlink, or (in long format) any non-directory
            if long_format {
                let file_name = path.display().to_string();
                print_long_entry(&meta, &file_name, append_types, Some(&path))?;
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

fn print_short_format(
    entries: &[fs::DirEntry],
    show_all: bool,
    append_types: bool,
) -> Result<(), String> {
    let mut display_entries = Vec::new();

    if show_all {
        display_entries.push(".".to_string());
        display_entries.push("..".to_string());
    }

    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        let file_name_str = if append_types {
            append_type_suffix(entry, file_name_str.to_string())?
        } else {
            file_name_str.into_owned()
        };

        display_entries.push(quote_filename(&file_name_str));
    }

    // Sort like before
    display_entries.sort_by(|a, b| match (a.as_str(), b.as_str()) {
        (".", _) => std::cmp::Ordering::Less,
        (_, ".") => std::cmp::Ordering::Greater,
        ("..", _) if b != "." => std::cmp::Ordering::Less,
        (_, "..") if a != "." => std::cmp::Ordering::Greater,
        _ => {
            let key_a = a.trim_start_matches('.').to_lowercase();
            let key_b = b.trim_start_matches('.').to_lowercase();
            key_a.cmp(&key_b)
        }
    });

    // --- NEW: column layout ---
    let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

    // Find longest entry
    let max_len = display_entries.iter().map(|s| s.len()).max().unwrap_or(1);
    let col_width = max_len + 2; // add spacing
    let cols = std::cmp::max(1, term_width / col_width);
    let rows = (display_entries.len() + cols - 1) / cols;

    for row in 0..rows {
        for col in 0..cols {
            let idx = col * rows + row;
            if idx < display_entries.len() {
                let name = &display_entries[idx];
                print!("{:<width$}", name, width = col_width);
            }
        }
        println!();
    }
    Ok(())
}

// Modified print_long_format to pass the directory path
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
        if let Ok(meta) = fs::symlink_metadata(path) {
            total_blocks += meta.blocks();
            display_entries.push((meta, ".".to_string()));
        }
        if let Ok(meta) = fs::symlink_metadata(path.join("..")) {
            total_blocks += meta.blocks();
            display_entries.push((meta, "..".to_string()));
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

        // let file_name_str = if append_types {
        //     append_type_suffix(entry, file_name_str.to_string())?
        // } else {
        //     file_name_str.into_owned()
        // };

        // display_entries.push((metadata, file_name_str));

        let raw_name = file_name_str.into_owned();
        display_entries.push((metadata, raw_name));
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
        // Pass the directory path so we can construct correct full paths
        // print_long_entry(&metadata, &file_name, append_types, Some(path))?;

        let full_path = path.join(&file_name);
        print_long_entry(&metadata, &file_name, append_types, Some(&full_path))?;
    }

    Ok(())
}

// Print a single file entry in long format (like `ls -l`), with permissions, ownership, size, time, and symlink target.
fn print_long_entry(
    metadata: &fs::Metadata,
    file_name: &str,
    append_types: bool,
    full_path: Option<&std::path::Path>,
) -> Result<(), String> {
    // use std::os::unix::fs::MetadataExt;

    let ftype = file_type_char(metadata.mode());
    // let perms = format_mode(metadata.mode());
    let perms = format_mode(
        metadata.mode(),
        full_path.unwrap_or(std::path::Path::new(file_name)),
    );

    let links = metadata.nlink();
    let owner = get_owner(metadata.uid()).unwrap_or_else(|_| String::from("unknown"));
    let group = get_group(metadata.gid()).unwrap_or_else(|_| String::from("unknown"));

    let mtime = metadata.modified().unwrap_or(SystemTime::now());
    let datetime: DateTime<Local> = mtime.into();

    let current_year = Local::now().year();
    let mtime_year = datetime.year();

    let mtime_str = if mtime_year == current_year {
        datetime.format("%b %e %H:%M").to_string()
    } else {
        datetime.format("%b %e  %Y").to_string()
    };

    let file_type = metadata.mode() & libc::S_IFMT;
    let size_or_dev = if file_type == libc::S_IFCHR || file_type == libc::S_IFBLK {
        let rdev = metadata.rdev();
        format!("{}, {}", major(rdev), minor(rdev))
    } else {
        format!("{}", metadata.size())
    };

    // let mut display_name = file_name.to_string();
    let mut display_name = quote_filename(file_name);

    if append_types {
        if let Some(path) = full_path {
            display_name = append_type_suffix_for_path(path, display_name)?;
        }
    }

    print!(
        "{}{} {:>3} {:>8} {:>8} {:>8} {} {}",
        ftype, perms, links, owner, group, size_or_dev, mtime_str, display_name
    );

    // --- FIX: follow symlink and append suffix based on target ---
    if ftype == 'l' {
        if let Some(path) = full_path {
            if let Ok(target) = fs::read_link(path) {
                let mut target_str = target.display().to_string();

                if append_types {
                    if let Ok(target_meta) = fs::metadata(path) {
                        if target_meta.is_dir() {
                            target_str.push('/');
                        } else if target_meta.file_type().is_socket() {
                            target_str.push('=');
                        } else if target_meta.file_type().is_fifo() {
                            target_str.push('|');
                        } else if target_meta.is_file()
                            && target_meta.permissions().mode() & 0o111 != 0
                        {
                            target_str.push('*');
                        }
                    }
                }

                print!(" -> {}", target_str);
            }
        }
    }

    println!();
    Ok(())
}

// Append special characters to file names based on their type (for `-F`)
fn append_type_suffix(entry: &fs::DirEntry, file_name: String) -> Result<String, String> {
    let file_type = entry
        .file_type()
        .map_err(|e| format!("Error getting file type: {}", e))?;

    let mut file_name_str = file_name;

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
    // Use symlink_metadata to avoid following symlinks
    let metadata = match fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(_) => {
            // If we can't get metadata, return the original filename without modification
            return Ok(file_name);
        }
    };

    let mut file_name_str = file_name;

    if metadata.is_dir() {
        file_name_str.push('/');
    } else if metadata.file_type().is_fifo() {
        file_name_str.push('|');
    } else if metadata.file_type().is_socket() {
        file_name_str.push('=');
    } else if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
        file_name_str.push('*');
    }
    // else if metadata.file_type().is_symlink() {
    //     file_name_str.push('@');
    // }

    Ok(file_name_str)
}

// Check if a file is executable (for the `-F` flag)
fn is_executable(entry: &fs::DirEntry) -> Result<bool, String> {
    let metadata = entry
        .metadata()
        .map_err(|e| format!("Error getting metadata: {}", e))?;

    Ok(metadata.permissions().mode() & 0o111 != 0)
}

fn format_mode(mode: u32, path: &std::path::Path) -> String {
    let mut perms = String::new();

    // user permissions
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o100 != 0 {
        if mode & 0o4000 != 0 {
            's'
        } else {
            'x'
        }
    } else {
        if mode & 0o4000 != 0 {
            'S'
        } else {
            '-'
        }
    });

    // group permissions
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o010 != 0 {
        if mode & 0o2000 != 0 {
            's'
        } else {
            'x'
        }
    } else {
        if mode & 0o2000 != 0 {
            'S'
        } else {
            '-'
        }
    });

    // others permissions
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o001 != 0 {
        if mode & 0o1000 != 0 {
            't'
        } else {
            'x'
        }
    } else {
        if mode & 0o1000 != 0 {
            'T'
        } else {
            '-'
        }
    });

    // 🔹 check for extended attributes
    if let Ok(mut attrs) = xattr::list(path) {
        if attrs.next().is_some() {
            perms.push('+');
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

fn file_type_char(mode: u32) -> char {
    match mode & libc::S_IFMT {
        libc::S_IFDIR => 'd',  // directory
        libc::S_IFCHR => 'c',  // character device
        libc::S_IFBLK => 'b',  // block device
        libc::S_IFLNK => 'l',  // symlink
        libc::S_IFIFO => 'p',  // named pipe (FIFO)
        libc::S_IFSOCK => 's', // socket
        libc::S_IFREG => '-',  // regular file
        _ => '?',              // unknown
    }
}

fn quote_filename(name: &str) -> String {
    // Characters that need quoting (shell metacharacters + whitespace)
    let special_chars = [
        ' ', '\t', '\n', '\'', '"', '\\', '[', ']', '{', '}', '(', ')', '*', '?', '~', '!', '$',
        '&', ';', '|', '<', '>', '`',
    ];

    let needs_quotes = name.chars().any(|c| special_chars.contains(&c));

    if needs_quotes {
        format!("'{}'", name)
    } else {
        name.to_string()
    }
}

fn expand_path(path: &str) -> std::path::PathBuf {
    if path == "~" || path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            if path == "~" {
                return home;
            } else {
                return home.join(&path[2..]);
            }
        }
    }
    std::path::PathBuf::from(path)
}
