#![allow(unused)]
use std::{fs, io, path::Iter};


fn main() -> Result<(), io::Error>{
    let input = fs::read_to_string("./input.txt")?;
    let result: u32 = input
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
            let left: Vec<u32> = left.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<u32>().unwrap()).collect();
            let right: Vec<u32> = right.replace("  ", " ").split(" ").into_iter().map(|num| num.parse::<u32>().unwrap()).collect();
            (left, right)
        })
        .map(|(left, right)| {
            let mut sum: u32 = 0;
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
        .sum();
    println!("task 1: {result}");

    Ok(())
}
