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
    // here i used unwrap_or to provide a default valuue , in casee there is noo secound arg it wont panic and triyablna l7aflaa !!
    let source = &args[..args.len() - 1];
    if !dest.exists() {
        return Err(format!("cp: target '{}' does not exist", dest.display()));
    }

    for src in source {
        // here i will checked if the source file exists or not
        let src_path = Path::new(src);

        if !src_path.exists() {
            println!("cp: cannot stat '{}': No such file or directory", src);
            continue;
        }
        // now it exists so we gonna check if the dest is a dir or a file
        let target = if dest.is_dir() {
            // if its a dirictory we willl copy the file inside it with the same name
            if let Some(file_name) = src_path.file_name() {
                dest.join(file_name)
            } else {
    //  here if the source path is not a valid file name (like if its a directory or empty)
                println!("cp: invalid source file '{}'", src);
                continue;
            }
        } else {
    // here in case itss a file we gonna overwrite it with the source file, 

            // let target = if dest.is_dir() {
            //     dest.join(src_path.file_name().unwrap()
        // to_path_buf() // convert the Path to a PathBuf so we can use it in fs::copy
        // the perpese 
            dest.to_path_buf()
        };
    //  here we'ree sure that the scr is a file and the destination is valid so we can copy it, 
        if let Err(e) = fs::copy(&src_path, &target) {
            println!("cp: failed to copy file '{}': {}", src, e);
        }
        // and of coursee we should handle any type of tmajninat that might occur fee had l lprocess w naprintiwha bla panic

    }

    Ok(())
}
