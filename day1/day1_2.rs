use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ./day1_2 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();

    let mut accumulator = 0;
    for line_result in lines {
        let line = line_result?;
        let first_num = get_first_number_from_string(&line, false);
        let last_num = get_first_number_from_string(&line, true);

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let calibration_value_str = format!("{}{}", first, last);
            if let Ok(calibration_value) = calibration_value_str.parse::<u32>() {
                println!("{}", calibration_value);
                accumulator += calibration_value;
            }
        }
    }

    println!("{}", accumulator);
    Ok(())
}

fn get_first_number_from_string(input: &str, reverse: bool) -> Option<i32> {
    let numbers_to_find = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "eno", "owt", "eerht", "ruof", "evif","xis", "neves", "thgie", "enin"];
    let input_str = if reverse {
        input.chars().rev().collect::<String>()
    } else {
        input.to_string()
    };
    let mut lowest_index = input_str.len();
    let mut candidate: Option<&str> = None;

    for &n in numbers_to_find.iter() {
        if let Some(index) = input_str.find(n) {
            if index < lowest_index {
                lowest_index = index;
                candidate = Some(n);
            }
        }
    }

    match candidate {
        Some("one") | Some("eno") => Some(1),
        Some("two") | Some("owt") => Some(2),
        Some("three") | Some("eerht") => Some(3),
        Some("four") | Some("ruof") => Some(4),
        Some("five") | Some("evif") => Some(5),
        Some("six") | Some("xis") => Some(6),
        Some("seven") | Some("neves") => Some(7),
        Some("eight") | Some("thgie") => Some(8),
        Some("nine") | Some("enin") => Some(9),
        Some(n) => n.parse().ok(),
        None => None,
    }
}
