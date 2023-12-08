#![allow(unused)]
use std::{fs, os::windows::io::HandleOrInvalid, collections::HashMap};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input_demo.txt")?;
    let directions = parse_directions(input.lines().nth(0).unwrap());
    let dir_map: HashMap<&str, &str> = HashMap::new();

    // for line in input.lines() {

    // }

    Ok(())
}


enum Direction {
    Left,
    Right,
}

fn parse_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|ch| {
            if ch == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let (key, val) = line.split_once(" = ").unwrap();
    let val = val.replace("(", "");
    let val = val.replace(")", "");
    let (val_left, val_right) = val.split_once(", ").unwrap();

    (key.to_owned(), (val_left.to_owned(), val_right.to_owned()))
}
