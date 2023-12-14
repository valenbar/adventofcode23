

fn main() -> Result<(), std::io::Error> {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);

    Ok(())
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>().to_vec();
    let patterns = lines
        .split(|l| **l == "".to_string())
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut hline_sum: usize = 0;
    let mut vline_sum: usize = 0;
    for pattern in &patterns {
        dbg!(pattern);
        // check horizontal mirroring
        let h_windows = pattern.windows(2);
        let pair_lines = h_windows.enumerate().filter(|(_, win)| win[0] == win[1]);

        let mut mirrored_horizontally = false;
        for (pair_index, _) in pair_lines {
            mirrored_horizontally = {
                let paired_lines: Vec<(&&str, &&str)> = {
                    let start_top = pair_index;
                    let start_bot = pair_index + 1;
                    let a = &pattern[0..start_top];
                    let b = &pattern[start_bot + 1..];
                    a.iter().rev().zip(b.iter()).collect::<Vec<(&&str, &&str)>>()
                };
                dbg!(&paired_lines);
                paired_lines.iter().all(|(a, b)| a == b)
            };

            if mirrored_horizontally {
                hline_sum += pair_index + 1;
                break;
            }
        }
        if mirrored_horizontally {
            continue;
        }

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

        let h_windows = pattern_tr.windows(2);
        let pair_lines = h_windows.enumerate().filter(|(_, win)| win[0] == win[1]);

        let mut mirrored_vertically = false;
        for (pair_index, _) in pair_lines {
            mirrored_vertically = {
                let paired_lines: Vec<(&&str, &&str)> = {
                    let start_top = pair_index;
                    let start_bot = pair_index + 1;
                    let a = &pattern_tr[0..start_top];
                    let b = &pattern_tr[start_bot + 1..];
                    a.iter().rev().zip(b.iter()).collect::<Vec<(&&str, &&str)>>()
                };
                paired_lines.iter().all(|(a, b)| a == b)
            };

            // if yes then:
            if mirrored_vertically {
                vline_sum += pair_index + 1;
                break;
            }
        }
        if mirrored_vertically {
            continue;
        }
        panic!("no horizontal nor vertical mirroring detected");
    }

    return hline_sum * 100 + vline_sum;
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
        dbg!(result);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("./input.txt");
        let result = part1(input);
        assert_eq!(result, 36041);
    }
}
