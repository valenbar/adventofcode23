#![allow(unused)]
use itertools::Itertools;
use phf::phf_map;
use std::{collections::HashMap, fs, iter::zip};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|s| s.split_once(" "))
        .map(Option::unwrap)
        .map(Hand::from)
        .collect();

    hands.sort();
    // for hand in &hands { println!("{:?}", hand); }

    let task_1: usize = hands.iter()
        .enumerate()
        .map(|(mult, hand)| hand.bid * (mult + 1) )
        .sum();

    println!("{}", task_1);

    Ok(())
}

static CARD_STRENGTH: phf::Map<char, usize> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'J' => 9,
    'T' => 8,
    '9' => 7,
    '8' => 6,
    '7' => 5,
    '6' => 4,
    '5' => 3,
    '4' => 2,
    '3' => 1,
    '2' => 0,
};

#[derive(Debug, Eq, PartialEq, Ord)]
struct Hand {
    hand_type: usize,
    card_strength: Vec<usize>,
    bid: usize,
}

impl From<(&str, &str)> for Hand {
    fn from(value: (&str, &str)) -> Self {
        let (hand_str, bid_str) = value;
        // parse hand type
        let occurence_vec: Vec<usize> = hand_str
            .chars()
            .unique()
            .map(|ch1| hand_str.chars().filter(|ch2| ch1 == *ch2).count())
            .sorted()
            .rev()
            .collect();

        let hand_type = match occurence_vec.as_slice() {
            [5] => 7,
            [4, ..] => 6,
            [3, 2] => 5,
            [3, ..] => 4,
            [2, 2, ..] => 3,
            [2, ..] => 2,
            [1, ..] => 1,
            _ => panic!("hand type couldn't be detected"),
        };

        // parse hand strength
        let card_strenght: Vec<usize> = hand_str
            .chars()
            .map(|ch| CARD_STRENGTH.get(&ch))
            .map(|n| *n.unwrap())
            .collect();

        let bid = bid_str.parse::<usize>().unwrap();

        // return Hand
        Hand {
            hand_type,
            card_strength: card_strenght,
            bid,
        }
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
    fn gt(&self, other: &Self) -> bool {
        self.hand_type > other.hand_type || ( self.hand_type == other.hand_type && {
            for (a, b) in zip(self.card_strength.clone(), other.card_strength.clone()) {
                if a == b { continue; }
                if a > b { return true; }
                if a < b { return false; }
            }
            false
        })
    }
    fn lt(&self, other: &Self) -> bool {
        !(self > other)
    }
}

