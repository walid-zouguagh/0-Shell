use std::env;

pub fn run(args: &[String]) -> Result<(), String> {
    //    for arg in args

    if args.len() > 1 {
        return Err("cd: too many arguments one is enogh".to_string());
    }

    let curent_path = env::current_dir().unwrap_or("/".into()).to_string_lossy().to_string();

    let target = if args.is_empty() || args[0] == "--" {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else if args[0].starts_with("~") {
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        if args[0] == "~" {
            home 
        } else {
            format!("{}{}", home, &args[0][1..])
        }
    } else if args[0] == "-" {
        match env::var("OLDPWD") {
            Ok(old) => {
                println!("{}", old);
                old
            }
            Err(_) => {
                return Err("cd: OLDPWD not set".to_string());
            }
        }
    } else {
        args[0].clone()
    };

    if let Err(e) = env::set_current_dir(&target) {
        return Err(format!("cd: {}: {}", target, e));
    }
    env::set_var("OLDPWD", curent_path);
    Ok(())
}
