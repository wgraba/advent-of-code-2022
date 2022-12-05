use std::{
    collections::{HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn calc_priority(item: char) -> u16 {
    assert!(item.is_ascii_alphabetic());

    if item.is_ascii_lowercase() {
        (item as u8 - 'a' as u8) as u16 + 1
    } else {
        (item as u8 - 'A' as u8) as u16 + 27
    }
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut total_priorities = 0;
    for input in input_reader.lines() {
        let items: Vec<char> = input.expect("Invalid puzzle input").chars().collect();
        let comp1: HashSet<&char> = HashSet::from_iter(&items[..items.len() / 2]);
        let comp2: HashSet<&char> = HashSet::from_iter(&items[items.len() / 2..]);

        let common_elements: Vec<_> = comp1.intersection(&comp2).collect();
        assert!(common_elements.len() == 1);

        total_priorities += calc_priority(**common_elements[0]);
        println!(
            "{}: {:?}; {:?} - {:?}",
            items.iter().collect::<String>(),
            comp1,
            comp2,
            common_elements,
        );
    }

    println!("Sum of Priorities: {total_priorities}");
}
