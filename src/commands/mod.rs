pub mod cd;
pub mod pwd;

pub type CmdResult = Result<(), String>;

pub fn dispatch(cmd: &str, args: &[String]) -> CmdResult {
    match cmd {
        "exit" => {
            let code = args.get(0)
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            std::process::exit(code);
        }
        // "echo" => echo::run(args),
        "cd"   => cd::run(args),
        "pwd"  => pwd::run(args),
        // "ls"   => ls::run(args),
        // "cat"  => cat::run(args),
        // "cp"   => cp::run(args),
        // "rm"   => rm::run(args),
        // "mv"   => mv::run(args),
        // "mkdir"=> mkdir::run(args),


        // "echo" => super::commands::echo::run(args),
        // "ls"   => super::commands::ls::run(args),
        // "cat"  => super::commands::cat::run(args),
        // "cp"   => super::commands::cp::run(args),
        // "rm"   => super::commands::rm::run(args),
        // "mv"   => super::commands::mv::run(args),
        // "mkdir"=> super::commands::mkdir::run(args),
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}
