use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

#[derive(Clone)]
struct Card {
    id: usize,
    winners: Vec<i32>,
    candidates: Vec<i32>,
    count: i32,
    score: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("./day4_2 <filename>");
        process::exit(1);
    }

    let file = File::open(&args[1])?;
    let lines = io::BufReader::new(file).lines();
    let mut cards: Vec<Card> = Vec::new();

    // Parse and load all cards
    for (index, line_read) in lines.enumerate() {
        let line = line_read?;
        let parts: Vec<&str> = line.split(':').collect();
        let more_parts: Vec<&str> = parts[1].split('|').collect();

        let card_id = index + 1;
        let card_winners: Vec<i32>  = more_parts[0].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
        let card_candidates: Vec<i32>  = more_parts[1].trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

        let card_entry = Card {
            id: card_id,
            winners: card_winners,
            candidates: card_candidates,
            count: 1,
            score: 0,
        };

        cards.push(card_entry);
    }

    // Update counters
    let cards_len = cards.len();
    for index in 0..cards_len {
        let (matches, score) = get_card_score(&cards[index]);
        cards[index].score = score;

        if matches > 0 {
            for i in index + 1..std::cmp::min(index + 1 + matches as usize, cards_len) {
                cards[i].count += 1 * cards[index].count;
            }
        }
    }

    // Count them all
    let mut card_amount = 0;
    for c in cards {
        //println!("Card {} - {:?} - {:?} {} {}", c.id, c.winners, c.candidates, c.score, c.count);
        card_amount +=  c.count;
    }

    println!("Total scratchcards: {}", card_amount);
    Ok(())
}

fn get_card_score(card: &Card) -> (i32, i32) {
    let mut score = 0;
    let mut matches = 0;
    for w in &card.winners {
        if card.candidates.contains(&w) {
            if score == 0 {
                score = 1;
            } else {
                score *=2;
            }

            matches += 1;
        }
    }

    (matches, score)
}