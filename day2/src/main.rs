use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Strategy {
    WIN,
    TIE,
    LOSE,
}

fn xlat_opponent_move(opponent_move: &str) -> Result<Move, ()> {
    match opponent_move {
        "A" => Ok(Move::ROCK),
        "B" => Ok(Move::PAPER),
        "C" => Ok(Move::SCISSORS),
        _ => Err(()),
    }
}

fn xlat_my_move(opponent_move: &Move, strategy: &Strategy) -> Move {
    match strategy {
        Strategy::WIN => {
            match opponent_move {
                Move::ROCK => Move::PAPER,
                Move::PAPER => Move::SCISSORS,
                Move::SCISSORS => Move::ROCK,
            }
        }
        Strategy::TIE => opponent_move.clone(),
        Strategy::LOSE => {
            match opponent_move {
                Move::ROCK => Move::SCISSORS,
                Move::PAPER => Move::ROCK,
                Move::SCISSORS => Move::PAPER,
            }
        }
    }
}

fn xlat_strategy(strategy: &str) -> Result<Strategy, ()> {
    match strategy {
        "X" => Ok(Strategy::LOSE),
        "Y" => Ok(Strategy::TIE),
        "Z" => Ok(Strategy::WIN),
        _ => Err(()),
    }
}

fn calc_my_round_score(opponent: &Move, me: &Move) -> u64 {
    (*me as u64)
        + match opponent {
            Move::ROCK if me == &Move::PAPER => 6,
            Move::PAPER if me == &Move::SCISSORS => 6,
            Move::SCISSORS if me == &Move::ROCK => 6,
            _ if opponent == me => 3,
            _ => 0,
        }
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut total_score = 0;
    for input in input_reader.lines() {
        let data_str = input.expect("Invalid input");
        let data: Vec<&str> = data_str.split(' ').collect();

        let opponent_move = xlat_opponent_move(data[0]).unwrap();
        let strategy = xlat_strategy(data[1]).unwrap();
        let my_move = xlat_my_move(&opponent_move, &strategy);

        let round_score = calc_my_round_score(&opponent_move, &my_move);
        total_score += round_score;

        println!("{opponent_move:#?} {strategy:#?} {my_move:#?} {round_score}")
    }

    println!("My total score is {total_score}");
}
