use std::{fs, collections::HashMap};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let mut lines = input.lines();
    let directions = parse_directions(lines.next().unwrap());
    let mut dir_map: HashMap<String, (String, String)> = HashMap::new();
    lines.next();

    for line in lines {
        let (key, value) = parse_line(line);
        dir_map.insert(key, value);
    }

    let task_1 = follow_directions(directions.clone(), dir_map.clone());
    println!("task 1: {task_1}");

    let task_2 = follow_directions_2(directions, dir_map);
    println!("task 2: {task_2}");

    Ok(())
}


fn follow_directions(directions: Vec<Direction>, dir_map: HashMap<String, (String, String)>) -> usize {
    let mut count_steps = 0;
    let mut current_location = "AAA".to_string();
    loop {
        for direction in &directions {
            current_location = match dir_map.get(&current_location) {
                Some((left, right)) => if *direction == Direction::Left { left.clone() } else { right.clone() },
                _ => panic!("something went wrong."),
            };
            count_steps += 1;
            if current_location == "ZZZ" {
                return count_steps;
            }
        }
    }
}


fn follow_directions_2(directions: Vec<Direction>, dir_map: HashMap<String, (String, String)>) -> usize {
    let mut count_steps = 0;

    let mut current_locations: Vec<String> = dir_map
        .keys()
        .map(|s| s.to_string())
        .filter(|s| s.ends_with('A'))
        .collect();

    loop {
        for direction in &directions {
            for location in current_locations.iter_mut() {
                let new_location = match dir_map.get(location) {
                    Some((left, right)) => if *direction == Direction::Left { left.clone() } else { right.clone() },
                    _ => panic!("something went wrong."),
                };
                *location = new_location.to_string();
            }
            count_steps += 1;
            if current_locations.iter().all(|l| l.ends_with('Z')) {
                return count_steps;
            }
        }
    }
}


#[derive(PartialEq, Clone)]
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
