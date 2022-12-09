use std::{
    collections::{HashMap, BTreeMap},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

struct TreeGrid {
    // max_point: Point,
    trees: BTreeMap<Point, u8>,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl TreeGrid {
    pub fn new() -> TreeGrid {
        TreeGrid {
            // max_point: Point { x: 0, y: 0 },
            trees: BTreeMap::new(),
        }
    }

    pub fn add_tree(&mut self, point: Point, height: u8) -> Option<u8> {
        self.trees.insert(point, height);
        Some(height)
    }

    pub fn is_visible(&self, point: Point, dir: Direction) -> Result<bool, ()> {
        let point_inc: (isize, isize) = match dir {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        };

        if let Some(base_tree_height) = self.trees.get(&point) {
            let mut is_visible = true;

            let mut next_point = Point {
                x: point.x + point_inc.0,
                y: point.y + point_inc.1,
            };
            while let Some(tree_height) = self.trees.get(&next_point) {
                next_point = Point {
                    x: next_point.x + point_inc.0,
                    y: next_point.y + point_inc.1,
                };
                if tree_height >= base_tree_height {
                    is_visible = false;
                    break;
                }
            }

            Ok(is_visible)
        } else {
            Err(())
        }
    }
}

fn main() {
    let puzzle_input = File::open("input").expect("Unable to open file");
    let input_reader = BufReader::new(puzzle_input);

    let mut tree_grid = TreeGrid::new();
    for (y, input) in input_reader.lines().enumerate() {
        let tree_line = input.unwrap();
        for (x, c) in tree_line.chars().enumerate() {
            tree_grid
                .add_tree(
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    c.to_digit(10).expect("Invalid tree height") as u8,
                )
                .expect("Error adding tree!");
        }
    }

    println!("Num trees: {}", tree_grid.trees.len());

    let mut num_visible = 0;
    for (point, _) in tree_grid.trees.iter() {
        for dir in vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ] {
            if tree_grid.is_visible(*point, dir).unwrap() {
                // println!("{point:#?}");
                num_visible += 1;
                break;
            }
        }
    }

    println!("Num trees visible: {}", num_visible);
}
