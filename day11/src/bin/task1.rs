use std::{ collections::VecDeque, iter, str::Chars, result };

use combinations::Combinations;

fn main() -> Result<(), std::io::Error> {
    Ok(())
}

fn part1(input: &str) -> usize {
    let expanded = expand(input);
    let locations = get_galaxy_locations(expanded);
    let location_combinations: Vec<Vec<Point>> = Combinations::new(locations, 2).collect();
    let dist_sum: usize = location_combinations
        .iter()
        .map(|pair| pair[0].dist(&pair[1]))
        .sum();
    for pair in location_combinations.iter() {
        dbg!(pair);
        let dist = pair[0].dist(&pair[1]);
        dbg!(dist);
    }
    dist_sum
}

fn expand(input: &str) -> Vec<String> {
    let result_horizontal = input
        .lines()
        .map(|line| {
            if line.contains("#") {
                vec![line.to_owned()]
            } else {
                let result = vec![line.to_owned(), line.to_owned()];
                result
            }
        })
        .flatten()
        .collect::<Vec<String>>();

    let map: Vec<Vec<char>> = input
        .lines()
        .map(str::chars)
        .map(Chars::collect::<Vec<char>>)
        .collect();

    let map = transpose(map);

    let map = map
        .iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let result_vertical = input
        .lines()
        .map(|line| {
            if line.contains("#") {
                vec![line.to_owned()]
            } else {
                let result = vec![line.to_owned(), line.to_owned()];
                result
            }
        })
        .flatten()
        .collect::<Vec<String>>();

    let result: Vec<Vec<char>> = input
        .lines()
        .map(str::chars)
        .map(Chars::collect::<Vec<char>>)
        .collect();

    let result = transpose(result);

    let result = result
        .iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    dbg!(&result);
    result
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v
        .into_iter()
        .map(|n| n.into_iter())
        .collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn get_galaxy_locations(map: Vec<String>) -> Vec<Point> {
    let mut locations: Vec<Point> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                locations.push(Point { x, y });
            }
        }
    }
    dbg!(&locations);
    locations
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, other: &Point) -> usize {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);
        delta_x + delta_y
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_demo_input() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
        );
        dbg!(result);
        assert_eq!(result, 374);
    }
}
