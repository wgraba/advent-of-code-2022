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

fn xlat_move(player_move: &str) -> Result<Move, ()> {
    match player_move {
        "A" | "X" => Ok(Move::ROCK),
        "B" | "Y" => Ok(Move::PAPER),
        "C" | "Z" => Ok(Move::SCISSORS),
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
        let moves_str = input.expect("Invalid input");
        let moves: Vec<&str> = moves_str.split(' ').collect();

        let opponent_move = xlat_move(moves[0]).unwrap();
        let my_move = xlat_move(moves[1]).unwrap();

        let round_score = calc_my_round_score(&opponent_move, &my_move);
        total_score += round_score;

        println!("{opponent_move:#?} {my_move:#?} {round_score}")
    }

    println!("My total score is {total_score}");
}
