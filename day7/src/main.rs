#![allow(unused)]
use itertools::Itertools;
use phf::phf_map;
use std::{collections::HashMap, fs, iter::zip};
mod hand_t1;
mod hand_t2;

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;

    // task 1
    let mut hands: Vec<hand_t1::Hand> = input
        .lines()
        .map(|s| s.split_once(" "))
        .map(Option::unwrap)
        .map(hand_t1::Hand::from)
        .collect();

    hands.sort();

    let task_1: usize = hands.iter()
        .enumerate()
        .map(|(mult, hand)| hand.bid * (mult + 1) )
        .sum();

    println!("task 1: {} - 248569531", task_1);

    // task 2
    let mut hands: Vec<hand_t2::Hand> = input
        .lines()
        .map(|s| s.split_once(" "))
        .map(Option::unwrap)
        .map(hand_t2::Hand::from)
        .collect();

    hands.sort();

    let task_2: usize = hands.iter()
        .enumerate()
        .map(|(mult, hand)| hand.bid * (mult + 1) )
        .sum();

    println!("task 2: {} - 250382098", task_2);


    Ok(())
}


