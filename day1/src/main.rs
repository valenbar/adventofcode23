use std::{fs};

fn get_lines(file: &str) -> Vec<String> {
    let content = fs::read_to_string(file).unwrap();
    content.lines().map(|s| s.to_string()).collect()
}

fn parse_line(line: &str) -> u32 {
    let mut result: u32 = 10;
    for char in line.chars() {
        if let Some(i) = char.to_digit(10) {
            result *= i;
            break;
        }
    }
    for char in line.chars().rev() {
        if let Some(i) = char.to_digit(10) {
            result += i;
            return result
        }
    }
    panic!("This shouldn't be reachable");
}

#[allow(unused)]
fn parse_line_words(line: &str) -> u32 {

    0
}

fn task_1(file: &str) -> u32 {
    let lines = get_lines(file);
    let mut sum = 0;
    for line in lines {
        let val = parse_line(&line);
        sum += val;
        // println!("{val}");
    }
    sum
}

#[allow(unused)]
fn task_2(file: &str) -> u32 {
    let lines = get_lines(file);
    let mut sum = 0;
    for line in lines {
        let val = parse_line_words(&line);
        sum += val;
        // println!("{val}");
    }
    sum

}

fn main() {
    println!("task 1: {}",
        task_1("./src/input.txt")
    );

    println!("task 2: {}",
        task_2("./src/input_demo_2.txt")
    )
}
