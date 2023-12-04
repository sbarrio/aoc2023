use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day4_1 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();
    let mut score_acum = 0;
    for line_read in lines {
        let line = line_read?;
        let parts: Vec<&str> = line.split(':').collect();
        let more_parts: Vec<&str> = parts[1].split('|').collect();

        let winners: Vec<i32>  = more_parts[0].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
        let candidates: Vec<i32>  = more_parts[1].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

        let mut score = 0;
        for w in winners {
            if candidates.contains(&w) {
                if score == 0 {
                    score = 1;
                } else {
                    score *=2;
                }
            }
        }

        score_acum += score;

        println!("Card score is {}", score);
    }

    println!("Final score is {}", score_acum);

    Ok(())
}