use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day3_1 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let read_lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut lines = read_lines.clone(); 

    for l in 0..lines.len() {
        let current_line = &lines[l];
    
        for (i, ch) in current_line.chars().enumerate() {
            if is_symbol(ch) {
                println!("Smybol: {} at {} {}", ch, l, i);

                // Prev line

                // Current line
                if (i > 0) {
                    find_number_in_line_from_index(&lines[l], i - 1);
                }
                if (i < current_line.len() - 1) {
                    find_number_in_line_from_index(&lines[l], i + 1);
                }
                
                // Next line
                //let test = &lines[l - 1][i-1..i];
                // if l > 0 && is_number(test) {
                //     find_number_in_line_from_index(&lines[l -1], i-1);
                // }
            }

 
        }
    }
    Ok(())
}

fn is_symbol(ch: char) -> bool {
    let reserved_chars: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    return !reserved_chars.contains(&ch);
}

fn is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn find_number_in_line_from_index(line: &str, start_index: usize) -> i32 {
    let numbers: Vec<&str> = line.split('.').filter(|&part| !part.is_empty()).collect();
    let mut found_match = "";

    for n in &numbers {
        // Skip processing symbols
        if !is_number(n) {
            continue;
        }

        if let Some(index) = line.find(n) {
            println!("Found '{}' at index {}", n, index);
            if index <= start_index && index + n.len() - 1 >= start_index {
                println!("This is the one {}", n);
                found_match = n;
                break;
            }
        } else {
            println!("'{}' not found in the string.", n);
        }
    }

    let match_result = found_match.parse::<i32>();
    match match_result {
        Ok(parsed_int) => {
            return parsed_int;
        }
        Err(_) => {
            println!("Failed to parse the string as an integer.");
            return -1;
        }
    }
}

