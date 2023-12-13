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
    static DIGIT_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let mut digits: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
            digits.push(d);
        } else {
            for (j, word) in DIGIT_WORDS.iter().enumerate() {
                if line[i..].starts_with(word) {
                    digits.push((j+1) as u32);
                }
            }
        }
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
