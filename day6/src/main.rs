#![allow(unused)]
use core::time;
use std::{collections::HashMap, fs, iter::zip, mem::Discriminant};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    // let (_, times, _, distances) = input.split(":");

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
    println!("task 1: {}", prod_win_count);

    Ok(())
}
