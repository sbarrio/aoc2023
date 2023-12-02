use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day2_2 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();
    let mut power_sum = 0;
    
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

        let mut max_red_game = 0;
        let mut max_green_game = 0;
        let mut max_blue_game = 0;

        for game in game_rounds {
            let cube_entry: Vec<&str> = game.split(',').collect();
            for cube in cube_entry {
                max_red_game = std::cmp::max(obtain_box_color_count("red", cube), max_red_game);
                max_green_game = std::cmp::max( obtain_box_color_count("green", cube), max_green_game);
                max_blue_game = std::cmp::max(obtain_box_color_count("blue", cube), max_blue_game);
            }
        }
        let game_power = max_red_game * max_green_game * max_blue_game;
        power_sum += game_power;
    }

    println!("Total power sum is: {}", power_sum);
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