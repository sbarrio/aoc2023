use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day6_1 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();

    let mut race_durations: Vec<i32> = Vec::new();
    let mut race_records: Vec<i32> = Vec::new();

    // Read input file - (Dumb Rust won't let me use indexes so let's loop twice)
    for line in lines {
        let read_line = line?;
        if read_line.contains("Time:") {
            race_durations = read_line
            .replace("Time:", "")
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        };

        if read_line.contains("Distance:") {
            race_records = read_line
            .replace("Distance:", "")
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        };
    }            

    // Calculate different ways to beat each race
    let mut result = 1;
    for i in 0..race_durations.len() {
        let mut ways_to_beat_record = 0;
        let record =  race_records[i];
        let duration = race_durations[i];

        for time_pressed in 0..duration {
            if time_pressed * (duration - time_pressed) > record {
                ways_to_beat_record += 1;
            }
        }

        result *= ways_to_beat_record;
    }

    println!("{}", result);
    Ok(())
}