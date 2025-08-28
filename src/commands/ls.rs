use std::fs;

pub fn run(args: &[String]) {
    if !args.is_empty() {
        eprintln!("ls: No such file or directory (os error 2)");
        return;
    }

    let entries = fs::read_dir(".").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}

// use std::fs;
// use std::io;
// use std::path::Path;

// pub fn run(args: &[String]) {
//     let path = if args.is_empty() {
//         "."
//     } else {
//         &args[0]
//     };

//     if let Err(e) = list_dir(path) {
//         eprintln!("ls: {}: {}", path, e);
//     }
// }

// fn list_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
//     let entries = fs::read_dir(path)?;

//     for entry in entries {
//         let entry = entry?;
//         let file_name = entry.file_name();
//         let file_name = file_name.to_string_lossy();
//         println!("{}", file_name);
//     }

//     Ok(())
// }
