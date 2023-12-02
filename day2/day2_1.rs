use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day2_1 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();

    let mut valid_game_id_sum = 0;
    
    for line_read in lines {
        let line = line_read?;
        let parts: Vec<&str> = line.split(':').collect();

        let game_id_result = parts[0].replace("Game ", "").parse();
        let mut game_id = 0;
        match game_id_result {
            Ok(parsed_int) => {
                game_id = parsed_int
            }
            Err(_) => {
                println!("Failed to parse the game id string as an integer.");
            }
        }
        let game_rounds: Vec<&str> = parts[1].split(';').collect();
        let mut valid_game = true;

        for game in game_rounds {
            let cube_entry: Vec<&str> = game.split(',').collect();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in cube_entry {
                red += obtain_box_color_count("red", cube);
                green += obtain_box_color_count("green", cube);
                blue += obtain_box_color_count("blue", cube);
            }

            if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                valid_game = false;
            }
        }

        if valid_game {
            valid_game_id_sum += game_id;
        }

    }

    println!("Valid id game sum is: {}", valid_game_id_sum);
    Ok(())
}

fn obtain_box_color_count(color: &str, string: &str) -> i32 {
    if string.contains(color) {
        let count_result: Result<i32, _> = string.replace(color, "").trim().parse();

        match count_result {
            Ok(parsed_int) => {
                return parsed_int;
            }
            Err(_) => {
                return 0;
            }
        }
    }

    return 0;
}