use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::path::Path;

pub fn cat(args: &[String]) -> Result<(), String> {
//  in case no args were provided with the cmd cat we just read from stdin 
// not that stdin is just a file desctiptor li kaylisni 3la input dyaal terminal !!!!
    if args.is_empty() {
         let stdin = io::stdin();
        let reader = stdin.lock();
        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("cat: error reading stdin: {}", e),
            }
        }
        return Ok(());
    }

    for filename in args {
        // we gonna use Path from std::path since we dont neeed to change the path or manipulate it
        let path = Path::new(filename);

        // the err is gonna be handled while we try to read the countent of the file  
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("cat: {}: No such file", filename);
                continue;  
            }
        };
    // here i shoooose to use BufReader over reading directly from file so can read it line by line !!!
        let reader = BufReader::new(file);
        // here i'm sure that the file exists so we can read it line by line

        for line in reader.lines() {

            match line {
                Ok(content) => println!("{}", content),
                Err(e) => println!("cat: error reading {}: {}", filename, e),
            }
//  here i'm stillll chcking for errs  while reading the the file content, because each line is treated separately and return its own result
        }
    }

    Ok(())
}
