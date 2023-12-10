use std::fs;
use eyre::Result;

fn main() -> Result<()> {
    let task_1 = solve_task_1("input.txt")?;
    println!("task 1: {task_1}");
    Ok(())
}

fn solve_task_1(input: &str) -> Result<isize> {
    let input = fs::read_to_string(input)?;
    let input_numbers = input
        .lines()
        .map(str::split_whitespace)
        .map(|line| {
            line.flat_map(str::parse::<isize>)
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();
    let guessed_numbers: Vec<isize> = input_numbers
        .iter()
        .map(|v| find_next_value(v.clone()))
        .collect::<Vec<isize>>();

    Ok(guessed_numbers.iter().sum::<isize>())
}


fn find_next_value(vec: Vec<isize>) -> isize {
    let mut rows: Vec<Vec<isize>> = Vec::new();
    rows.push(vec.clone());
    for i in 0..vec.len() {
        let mut row: Vec<isize> = Vec::new();
        for j in 0..(rows[i].len() - 1) {
            let diff = (rows[i][j+1] - rows[i][j]) as isize;
            row.push(diff);
        }
        if row.iter().all(|n| *n == 0) {
            row.push(0);
            rows.push(row);
            break;
        }
        rows.push(row);
    }

    for i in (1..rows.len()).rev() {
        let diff: isize = *rows[i].last().unwrap();
        let last: isize = *rows[i-1].last().unwrap();
        rows[i-1].push(last + diff);
    }

    return *rows[0].last().unwrap();
}


#[test]
fn test_task_1_demo() -> Result<()> {
    let result = solve_task_1("input_demo.txt")?;
    assert_eq!(result, 114);
    Ok(())
}

#[test]
fn test_task_1_real() -> Result<()> {
    let result = solve_task_1("input.txt")?;
    assert_eq!(result, 2174807968);
    Ok(())
}