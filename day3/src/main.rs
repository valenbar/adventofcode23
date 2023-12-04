#![allow(unused)]

use std::{fs, io, vec};



fn parse_number(line: &Vec<char>, pos: usize) -> Option<u32> {
    let mut digits: Vec<char> = Vec::new();
    if let Some(_) = line[pos].to_digit(10) {
        digits.push(line[pos]);
    } else {
        return None;
    }

    for i in (0..pos).rev() {
        if let Some(_) = line[i].to_digit(10) {
            digits.push(line[i]);
        } else {
            break;
        }
    }
    digits.reverse();

    for i in (pos+1)..line.len() {
        if let Some(_) = line[i].to_digit(10) {
            digits.push(line[i]);
        } else {
            break;
        }
    }

    if let Ok(number) = digits.iter().collect::<String>().parse::<u32>() {
        Some(number)
    } else {
        None
    }
}


fn find_numbers(mat: &Vec<Vec<char>>) -> Vec<u32> {
    static NO_SYMBOL: &str = "0123456789.";
    let mut numbers = Vec::new();
    let mat_size = mat.len();

    for row in 0..mat_size {
        for col in 0..mat_size {
            let symbol = mat[row][col];

            if NO_SYMBOL.contains(symbol) {
                continue;
            }
            // print!("{symbol}");

            let (dl, ll, ul, uu, ur, rr, dr, dd) = (
                ((row + 1), (col - 1)),
                (row, (col - 1)),
                ((row - 1), (col - 1)),
                ((row - 1), col),
                ((row - 1), (col + 1)),
                (row, (col + 1)),
                ((row + 1), (col + 1)),
                ((row + 1), col),
            );

            let mut locations_to_check: Vec<(usize, usize)> = vec![ll, rr];
            if let Some(_) = mat[uu.0][uu.1].to_digit(10) {
                locations_to_check.push(uu);
            } else {
                locations_to_check.push(ul);
                locations_to_check.push(ur);
            }

            if let Some(_) = mat[dd.0][dd.1].to_digit(10) {
                locations_to_check.push(dd);
            } else {
                locations_to_check.push(dl);
                locations_to_check.push(dr);
            }


            // println!("{:?}", adj_locations);

            // enforce bounds
            for (x, y) in &locations_to_check {
                if *x >= mat_size || *y >= mat_size {
                    panic!("symbol on the border");
                }
            }

            for (r, c) in locations_to_check {
                if let Some(number) = parse_number(&mat[r as usize], c as usize) {
                    numbers.push(number);
                }
            }
        }
    }

    numbers
}


fn main() -> Result<(), io::Error>{
    let input: Vec<Vec<char>> = fs::read_to_string("./input_demo.txt")?
        .lines()
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    // task 1
    let numbers = find_numbers(&input);
    let result_task_1: u32 = numbers.iter().sum();
    println!("task 1: {result_task_1}");

    Ok(())
}
