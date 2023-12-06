#![allow(unused)]
use core::time;
use std::{collections::HashMap, fs, iter::zip, mem::Discriminant};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    // let (_, times, _, distances) = input.split(":");

    let prod_win_count = task_1(input.clone());
    println!("task 1: {}", prod_win_count);

    let prod_win_count = task_2(input);
    println!("task 2: {}", prod_win_count);

    Ok(())
}

fn task_1(input: String) -> usize {
    let times: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    let records: Vec<usize> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    let mut prod_win_count = 1;
    for (time, record) in zip(times, records) {
        let mut count_possible_wins = 0;
        for ms in 1..time {
            let travelled = ms * (time - ms);
            if travelled > record {
                count_possible_wins += 1;
            }
        }
        prod_win_count *= count_possible_wins;
    }
    prod_win_count
}

fn task_2(input: String) -> usize {
    let time: usize = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let record: usize = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut count_possible_wins = 0;
    for ms in 1..time {
        let travelled = ms * (time - ms);
        if travelled > record {
            count_possible_wins += 1;
        }
    }
    count_possible_wins
}
