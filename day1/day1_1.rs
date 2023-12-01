use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day1_1 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();
    
    let mut accumulator = 0;
    for line_result in lines {
        let line = line_result?;
        let first_num = get_first_number_char_from_string(&line);
        let reversed_line: String = line.chars().rev().collect();
        let last_num = get_first_number_char_from_string(&reversed_line);

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let calibration_value_str = format!("{}{}", first, last);
            if let Ok(calibration_value) = calibration_value_str.parse::<u32>() {
                accumulator += calibration_value;
            }
        }
    }

    println!("{}", accumulator);
    Ok(())
}

fn get_first_number_char_from_string(input: &str) -> Option<char> {
    for ch in input.chars() {
        if ch.is_digit(10) {
            return Some(ch);
        }
    }
    None
}
