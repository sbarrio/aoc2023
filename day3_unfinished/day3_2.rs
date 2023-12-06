use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day3_2 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();
    
    for line_read in lines {
        let line = line_read?;
        // ...

    }
    Ok(())
}

