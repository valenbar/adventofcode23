use core::panic;

use itertools::{Itertools, iproduct};

fn main() -> Result<(), std::io::Error> {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);

    Ok(())
}

fn part2(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map(str::to_owned).collect::<Vec<String>>().to_vec();
    let patterns = lines
        .split(|l| **l == "".to_string())
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<String>>>();

    let mut hline_sum: usize = 0;
    let mut vline_sum: usize = 0;

    for pattern_original in &patterns {
        let mut mirror_pattern_found = false;

        let pattern_indices = iproduct!(0..pattern_original.len(), 0..pattern_original[0].len()).collect_vec();
        dbg!(&pattern_indices);
        for (i, j) in pattern_indices {
            // flip possible smudge in pattern
            let mut pattern = pattern_original.clone();
            dbg!(&pattern);
            if pattern[i].chars().nth(j).unwrap().to_string() == "." {
                pattern[i].replace_range(j..=j, "#");
            } else {
                pattern[i].replace_range(j..=j, ".");
            }
            dbg!(&pattern);
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
            let pattern_tr = pattern_tr.iter().cloned().collect::<Vec<String>>();

            // let mut mirror_pattern_found = false;

            for (pattern, sum) in [(pattern, &mut hline_sum), (pattern_tr, &mut vline_sum)] {
                let h_windows = pattern.windows(2);
                let pair_lines = h_windows.enumerate().filter(|(_, win)| win[0] == win[1]);

                // todo find pattern with smudge
                for (pair_index, _) in pair_lines {
                    mirror_pattern_found = {
                        let paired_lines: Vec<(&String, &String)> = {
                            let start_top = pair_index;
                            let start_bot = pair_index + 1;
                            let a = &pattern[0..start_top];
                            let b = &pattern[start_bot + 1..];
                            a.iter().rev().zip(b.iter()).collect::<Vec<(&String, &String)>>()
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
                dbg!(hline_sum, vline_sum);
                break;
            } else {
                panic!("no horizontal nor vertical mirroring detected");
            }
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

    // #[test]
    // fn test_real_input() {
    //     let input = include_str!("./input.txt");
    //     let result = part2(input);
    //     todo!()
    //     // assert_eq!(result, 36041);
    // }
}
