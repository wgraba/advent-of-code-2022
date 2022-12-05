use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

fn gen_range(section: &str) -> RangeInclusive<u16> {
    let limits: Vec<u16> = section
        .split("-")
        .map(|s| u16::from_str(s).unwrap())
        .collect();
    assert!(limits.len() == 2);

    let min_limit = limits[0];
    let max_limit = limits[1];

    RangeInclusive::new(min_limit, max_limit)
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut num_dups = 0;
    for line in input_reader.lines() {
        let input: Vec<String> = line
            .unwrap()
            .split(",")
            .map(|s| String::from_str(s).unwrap())
            .collect();
        assert!(input.len() == 2);

        let elf1 = gen_range(input[0].as_str());
        let elf2 = gen_range(input[1].as_str());

        let dup_assign = (elf1.start() <= elf2.start() && elf1.end() >= elf2.end())
            || (elf2.start() <= elf1.start() && elf2.end() >= elf1.end());

        if dup_assign {
            num_dups += 1;
        }

        println!("{:?}: {}", input, dup_assign);
    }

    println!("Number of duplicate assingments: {num_dups}");
}
