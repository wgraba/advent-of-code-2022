use std::{
    fs::File,
    io::{BufRead, BufReader},
};


fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);
    
    for input in input_reader.lines() {
        let mut buf: Vec<char> = Vec::new();
        let mut last_idx = 0;
        for (i, c) in input.unwrap().char_indices() {
            buf.push(c);
            if buf.len() > 4 {
                buf.remove(0);
            }

            if buf.len() >= 4 {
                let mut sorted_buf = buf.clone();
                sorted_buf.sort_unstable();
                sorted_buf.dedup();
                if sorted_buf.len() >= 4 {
                    last_idx = i + 1;
                    break;
                }
            }
        }

        if last_idx > 0 {
            println!("{last_idx}: {buf:?}");
        } else {
            println!("No start of packet found!");
        }
    }
}
