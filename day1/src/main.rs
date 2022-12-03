use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Elf {
    id: u32,
    total_cals: u64,
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut elves: Vec<Elf> = Vec::new();
    let mut num_elves = 0;
    let mut num_cals = 0;
    for input in input_reader.lines() {
        let cal_input = input.expect("Error reading file");
        if cal_input == "" {
            num_elves += 1;

            let elf = Elf {
                id: num_elves,
                total_cals: num_cals,
            };
            elves.push(elf);

            num_cals = 0;
        } else {
            num_cals += cal_input
                .parse::<u64>()
                .expect(format!("Could not parse {cal_input} to number").as_str());
        }
    }

    elves.sort_by(|a, b| b.total_cals.cmp(&a.total_cals));

    let max_elf = &elves[0];
    println!(
        "Elf {} has the most calories at {}",
        max_elf.id, max_elf.total_cals
    );
}
