use std::process::Output;

fn main() -> Result<(), std::io::Error> {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

    Ok(())
}

fn part1(input: &str) -> usize {
    // dbg!(input);
    // todo!()
    405
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_input() {
        let result = part1(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        );
        assert_eq!(result, 405);
    }
}