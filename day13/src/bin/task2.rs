use std::borrow::{Borrow, BorrowMut};



fn main() -> Result<(), std::io::Error> {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

    Ok(())
}

fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>().to_vec();
    let patterns = lines
        .split(|l| **l == "".to_string())
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut hline_sum: usize = 0;
    let mut vline_sum: usize = 0;

    for pattern in &patterns {
        // transpose pattern for checking vertical mirroring
        let pattern_tr = {
            let mut mat: Vec<Vec<char>> = vec![vec!['0'; pattern.len()]; pattern[0].len()];
            for (i, line) in pattern.iter().enumerate() {
                for (j, ch) in line.chars().enumerate() {
                    mat[j][i] = ch;
                }
            }
            let transposed = mat.iter()
                .map(|vec| vec.iter().cloned().collect::<String>())
                .collect::<Vec<String>>();
            transposed
        };
        let pattern_tr = pattern_tr.iter().map(String::as_str).collect::<Vec<&str>>();

        let mut mirror_pattern_found = false;

        for (pattern, sum) in [(pattern, &mut hline_sum), (&pattern_tr, &mut vline_sum)] {
            let h_windows = pattern.windows(2);
            let pair_lines = h_windows.enumerate().filter(|(_, win)| win[0] == win[1]);

            // todo find pattern with smudge
            for (pair_index, _) in pair_lines {
                mirror_pattern_found = {
                    let paired_lines: Vec<(&&str, &&str)> = {
                        let start_top = pair_index;
                        let start_bot = pair_index + 1;
                        let a = &pattern[0..start_top];
                        let b = &pattern[start_bot + 1..];
                        a.iter().rev().zip(b.iter()).collect::<Vec<(&&str, &&str)>>()
                    };
                    paired_lines.iter().all(|(a, b)| a == b)
                };

                if mirror_pattern_found {
                    *sum += pair_index + 1;
                    break;
                }
            }
            if mirror_pattern_found {
                break;
            }
        }

        if mirror_pattern_found {
            continue;
        } else {
            panic!("no horizontal nor vertical mirroring detected");
        }
    }

    return hline_sum * 100 + vline_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_input() {
        let result = part2(
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
        dbg!(result);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("./input.txt");
        let result = part2(input);
        todo!()
        // assert_eq!(result, 36041);
    }
}
