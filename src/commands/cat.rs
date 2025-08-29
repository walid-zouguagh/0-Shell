use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn cat(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("cat: missing file operand".into());
    }

    for filename in args {
        let path = Path::new(filename);

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("cat: {}: No such file", filename);
                continue; // move to the next file
            }
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(e) => println!("cat: error reading {}: {}", filename, e),
            }
        }
    }

    Ok(())
}
