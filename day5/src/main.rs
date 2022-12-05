use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Move {
    start_stack_id: u32,
    num_moves: u32,
    end_stack_id: u32,
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut stack_ids: HashMap<u32, usize> = HashMap::new();
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    let mut lines = input_reader.lines();

    // Parse Stacks
    while let Some(Ok(line)) = lines.next() {
        if line.len() < 1 {
            break; // End of Stacks
        }
        for (i, c) in line.char_indices() {
            if c.is_ascii_alphabetic() {
                match stacks.get_mut(&i) {
                    Some(v) => {
                        v.push(c);
                    }
                    None => {
                        stacks.insert(i, vec![c]);
                    }
                };
            } else if c.is_ascii_digit() {
                stack_ids.insert(c.to_digit(10).unwrap(), i);
            }
        }
    }

    println!("Initial Stacks...");
    println!("{stack_ids:?}");
    println!("{stacks:?}");

    // Parse Moves
    let mut moves: Vec<Move> = Vec::new();
    for line in lines {
        let input = line.unwrap();
        let nums: Vec<u32> = input
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        assert!(nums.len() == 3);

        moves.push(Move {
            start_stack_id: nums[1],
            num_moves: nums[0],
            end_stack_id: nums[2],
        });
    }

    // Make Moves
    for mv in moves {
        println!("{mv:?}");

        for _ in 0..mv.num_moves {
            let el = stacks.get_mut(stack_ids.get(&mv.start_stack_id).unwrap()).unwrap().remove(0);
            stacks.get_mut(stack_ids.get(&mv.end_stack_id).unwrap()).unwrap().insert(0, el);
        }

        println!("{stacks:?}");
    }

    println!("Stacks after moves...");
    println!("{stacks:?}");

    // Print Stack Tops
    let mut ids = stack_ids.keys().collect::<Vec<_>>();
    ids.sort_unstable();

    for id in ids {
        println!(
            "{}: {:?}",
            id,
            stacks.get(stack_ids.get(id).unwrap()).unwrap().first()
        );
    }
}
