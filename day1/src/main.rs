use std::fs;

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

fn parse_all_digits(line: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    let mut i: usize = 0;

    while i < line.len() {
        let j = line.len() - i;
        if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
            digits.push(d);
            i += 1;
            continue;
        }
        if j >= 5 {
            if line[i..i+5].contains("three") {
                digits.push(3);
            }
            if line[i..i+5].contains("seven") {
                digits.push(7);
            }
            if line[i..i+5].contains("eight") {
                digits.push(8);
            }
        }
        if j >= 4 {
            if line[i..i+4].contains("four") {
                digits.push(4);
            }
            if line[i..i+4].contains("five") {
                digits.push(5);
            }
            if line[i..i+4].contains("nine") {
                digits.push(9);
            }
        }
        if j >= 3 {
            if line[i..i+3].contains("one") {
                digits.push(1);
            }
            if line[i..i+3].contains("two") {
                digits.push(2);
            }
            if line[i..i+3].contains("six") {
                digits.push(6);
            }
        }
        i += 1;
    }
    digits
}



fn parse_line_with_words(line: &str) -> u32 {
    let digits = parse_all_digits(line);
    let digit = digits.first().unwrap() * 10 + digits.last().unwrap();
    // println!("{}", digit);
    digit
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
        let val = parse_line_with_words(&line);
        sum += val;
    }
    sum

}

fn main() {
    println!("task 1: {}",
        task_1("./src/input.txt")
    );

    println!("task 2: {}",
        task_2("./src/input.txt")
    );

    // println!("{:?}", parse_all_digits("twosssdfsftwone"));
}
