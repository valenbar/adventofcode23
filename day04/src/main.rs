#![allow(unused)]
use std::{fs, io, path::Iter, collections::HashMap};


fn task_1(input: String) -> usize {
    input
        .lines()
        .map(|l|
            l.split_once(": ")
            .unwrap()
            .1
        )
        .map(|s|
            s.split_once(" | ")
            .unwrap()

        )
        .map(|(mut left, mut right)| {
            if left.starts_with(" ") {
                left = &left[1..];
            }
            if right.starts_with(" ") {
                right = &right[1..];
            }
            let left: Vec<usize> = left.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<usize>().unwrap()).collect();
            let right: Vec<usize> = right.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<usize>().unwrap()).collect();
            (left, right)
        })
        .map(|(left, right)| {
            let mut sum: usize = 0;
            for elem in left {
                if right.contains(&elem) {
                    if sum == 0 {
                        sum = 1;
                    } else {
                        sum *= 2;
                    }
                }
            }
            sum
        })
        .sum()
}


fn task_2(input: String) -> usize {
    let mut scratchcards: HashMap<usize, usize> = HashMap::new();
    for (card_nr, line) in input.lines().enumerate() {
        let line = line.split_once(": ").unwrap().1;
        let (mut left, mut right) = line.split_once(" | ").unwrap();
        if left.starts_with(" ") {
            left = &left[1..];
        }
        if right.starts_with(" ") {
            right = &right[1..];
        }
        let left: Vec<usize> = left.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<usize>().unwrap()).collect();
        let right: Vec<usize> = right.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<usize>().unwrap()).collect();
        let mut matches: usize = 0;
        for elem in left {
            if right.contains(&elem) {
                matches += 1;
            }
        }
        // compute scratchcard copies
        let current_copies = *scratchcards.entry(card_nr).or_insert(1);

        for next_card_nr in (card_nr + 1)..(card_nr + matches + 1) {
            let next_card_copies = scratchcards.entry(next_card_nr).or_insert(1);
            *next_card_copies += current_copies;
        }
    }
    // sum over scratchcards
    scratchcards.into_values().sum()
}


fn main() -> Result<(), io::Error>{
    let input = fs::read_to_string("./input.txt")?;

    // task 1
    let result: usize = task_1(input.to_owned());
    println!("task 1: {result}");

    // task 2
    let result: usize = task_2(input);
    println!("task 2: {result}");

    Ok(())
}
