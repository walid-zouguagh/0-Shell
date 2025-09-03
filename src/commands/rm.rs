use std::fs;
use std::path::Path;
use std::io;

pub fn rm(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("rm: missing operand".into());
    }
   let mut is_flage = false;
   let mut targets: Vec<String>  = Vec::new();
   for arg  in args  {
       if arg == "-r" {
        is_flage = true;
       }else {
           targets.push(arg.clone());
       }

   }
    if targets.is_empty(){
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
            if is_flage{
                if let Err(err) = remove_it_all(path){
                    print!("rm: failed to remove directory '{}': {}", target, err);
                }
            }else {
                println!("rm: cannot remove '{}': Is a directory", target);
            }
            // continue;
        }else {
            // daba we can ramove l file bla mashakiil 
            if let Err(e) = fs::remove_file(path) {
                println!("rm: failed to remove '{}': {}", target, e);
            }
        }
    }
    Ok(())
}




fn remove_it_all(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            remove_it_all(&child_path)?;
        }
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}

