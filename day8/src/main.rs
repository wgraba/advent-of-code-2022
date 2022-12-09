use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next_point(&self, dir: Direction) -> Point {
        match dir {
            Direction::UP => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::DOWN => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::LEFT => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::RIGHT => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

struct TreeGrid {
    // max_point: Point,
    trees: BTreeMap<Point, u8>,
}

#[derive(Copy, Clone)]
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

    pub fn num_visible(&self, tree: &Point, dir: Direction) -> Result<u32, ()> {
        if let Some(start_height) = self.trees.get(&tree) {
            let mut num_visible = 0;

            let mut next_pt = tree.next_point(dir);
            while let Some(tree_height) = self.trees.get(&next_pt) {
                num_visible += 1;
                if tree_height >= start_height {
                    break;
                }
                next_pt = next_pt.next_point(dir);
            }

            Ok(num_visible)
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

    // Part 1
    // let mut num_visible = 0;
    // for (point, _) in tree_grid.trees.iter() {
    //     for dir in vec![
    //         Direction::UP,
    //         Direction::DOWN,
    //         Direction::LEFT,
    //         Direction::RIGHT,
    //     ] {
    //         if tree_grid.is_visible(*point, dir).unwrap() {
    //             // println!("{point:#?}");
    //             num_visible += 1;
    //             break;
    //         }
    //     }
    // }

    // println!("Num trees visible: {}", num_visible);

    // Part 2
    let mut max_score = 0;
    for tree in tree_grid.trees.keys() {
        let num_up = tree_grid.num_visible(tree, Direction::UP).unwrap();
        let num_down = tree_grid.num_visible(tree, Direction::DOWN).unwrap();
        let num_left = tree_grid.num_visible(tree, Direction::LEFT).unwrap();
        let num_right = tree_grid.num_visible(tree, Direction::RIGHT).unwrap();

        let score = num_up * num_down * num_left * num_right;
        if score > max_score {
            max_score = score;
        }
    }

    println!("Max scenic score: {max_score}");
}
