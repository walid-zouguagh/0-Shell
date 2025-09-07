use std::fs::File;
use std::io::{ self, stdin, stdout, BufReader, Read };
use std::path::Path;
pub fn cat(args: &[String]) -> Result<(), String> {
    //  in case no args were provided with the cmd cat we just read from stdin
    // note that stdin is just a file desctiptor li kaylisni 3la l input dyaal terminal !!!!
    if args.is_empty() {
        // let stdin = io::stdin();
        // let reader = stdin.lock();
        // for line in reader.lines() {
        //     match line {
        //         Ok(content) => println!("{}", content),
        //         Err(e) => eprintln!("cat: error reading stdin: {}", e),
        //     }
        // }
        // return Ok(());
        let _ = io::copy(&mut stdin(), &mut stdout());
        return Ok(());
    }

    for filename in args {
        if filename == "-" {
            // let stdin = io::stdin();
            // let reader = stdin.lock();
            // for line in reader.lines() {
            //     match line {
            //         Ok(content) => println!("{}", content),
            //         Err(e) => eprintln!("cat: error reading stdin: {}", e),
            //     }
            // }
            let _ = io::copy(&mut stdin(), &mut stdout());

            // return Ok(());
        } else if filename == "--" {
                 println!("cat: {}: Not valid flag", filename);
                 continue;
        } else {
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
            let mut readdder = BufReader::new(file);
            let mut bufffer = Vec::new();
            if let Err(e) = readdder.read_to_end(&mut bufffer) {
                eprint!("cat: error reading file '{}': {}", filename, e);
                continue;
            }
            let content = String::from_utf8_lossy(&bufffer);
            println!("{}", content);
        }
    }
    Ok(())
}
