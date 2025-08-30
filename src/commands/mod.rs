pub mod cd;
pub mod pwd;
pub mod ls;
pub mod echo;
pub mod exit;
pub mod mkdir;
pub mod cat;
pub mod cp;
pub mod mv;
pub mod rm;
pub mod clear;

pub type CmdResult = Result<(), String>;

pub fn dispatch(cmd: &str, args: &[String]) -> CmdResult {
    match cmd {
        "exit" => exit::run(args),
        "echo" => echo::run(args),
        "cd"   => cd::run(args),
        "pwd"  => pwd::run(args),
        "ls"   => ls::run(args),
        "cat"  => cat::cat(args),
        "cp"   => cp::cp(args),
        "rm"   => rm::rm(args),
        "mv"   => mv::mv(args),
        "mkdir" => mkdir::mkdir(&args),
        "clear" => clear::clear(args),
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}