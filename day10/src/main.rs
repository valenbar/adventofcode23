use std::{fmt::Debug, fs};

use eyre::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let map = Map::from(input.as_str());
    // println!("Map: {:?}", map);
    let loop_len = map.loop_len();
    println!("loop length: {loop_len}");
    let task_1 = loop_len / 2;
    println!("task 1: {task_1}");
    Ok(())
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GROUND,
    START,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::GROUND,
            'S' => Tile::START,
            _ => panic!("Unknown symbol found"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    fn pos(&self, other: &Point) -> Pos {
        if self.y < other.y {
            return Pos::Above;
        }
        if self.y > other.y {
            return Pos::Below;
        }
        if self.x < other.x {
            return Pos::Left;
        }
        else {
            return Pos::Rigth;
        }
    }
}

#[derive(PartialEq, Eq)]
enum Pos {
    Above,
    Below,
    Left,
    Rigth,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Point,
}

impl Map {
    fn loop_len(&self) -> usize {
        let (x, y) = (self.start.x, self.start.y);
        let mut current_point = self.start.clone();
        let mut prev_point;

        let up;
        if y == 0 {
            up = None;
        } else {
            up = self.get_tile(Point { x: x, y: y-1 });
        }
        let down = self.get_tile(Point { x: x, y: y+1 });
        let left;
        if x == 0 {
            left = None;
        } else {
            left = self.get_tile(Point { x: x-1, y: y });
        }
        let right = self.get_tile(Point { x: x+1, y: y });

        if up.is_some() && [Tile::NS, Tile::SE, Tile::SW].contains(&up.unwrap()) {
            prev_point = current_point.clone();
            current_point = Point {
                x: current_point.x,
                y: current_point.y - 1,
            };
        } else if down.is_some() && [Tile::NE, Tile::NS, Tile::NW].contains(&down.unwrap()) {
            prev_point = current_point.clone();
            current_point = Point {
                x: current_point.x,
                y: current_point.y + 1,
            };
        } else if left.is_some() && [Tile::EW, Tile::NE, Tile::SE].contains(&left.unwrap()) {
            prev_point = current_point.clone();
            current_point = Point {
                x: current_point.x - 1,
                y: current_point.y,
            };
        } else if right.is_some() && [Tile::EW, Tile::NW, Tile::SW].contains(&right.unwrap()) {
            prev_point = current_point.clone();
            current_point = Point {
                x: current_point.x + 1,
                y: current_point.y,
            };
        } else {
            panic!("something went wrong");
        }

        let mut count_len = 1;
        loop {
            count_len += 1;
            let next_point = self.get_next_point(prev_point, current_point);
            if self.get_tile(next_point).unwrap() == Tile::START {
                // loop done
                return count_len;
            }
            prev_point = current_point.clone();
            current_point = next_point;
        }
    }

    fn get_next_point(&self, prev: Point, current: Point) -> Point {
        let prev_pos = prev.pos(&current);
        let (x, y) = (current.x, current.y);

        let up;
        if y == 0 {
            up = None;
        } else {
            up = self.get_tile(Point { x: x, y: y-1 });
        }
        let down = self.get_tile(Point { x: x, y: y+1 });
        let left;
        if x == 0 {
            left = None;
        } else {
            left = self.get_tile(Point { x: x-1, y: y });
        }
        let right = self.get_tile(Point { x: x+1, y: y });

        let current_tile = self.get_tile(current).unwrap();

        if up.is_some() && [Tile::NS, Tile::SE, Tile::SW, Tile::START].contains(&up.unwrap()) && prev_pos != Pos::Above && [Tile::NE, Tile::NS, Tile::NW].contains(&current_tile) {
            return Point {
                x: current.x,
                y: current.y - 1,
            };
        } else if down.is_some() && [Tile::NE, Tile::NS, Tile::NW, Tile::START].contains(&down.unwrap()) && prev_pos != Pos::Below && [Tile::NS, Tile::SE, Tile::SW].contains(&current_tile) {
            return Point {
                x: current.x,
                y: current.y + 1,
            };
        } else if left.is_some() && [Tile::EW, Tile::NE, Tile::SE, Tile::START].contains(&left.unwrap()) && prev_pos != Pos::Left && [Tile::EW, Tile::NW, Tile::SW].contains(&current_tile) {
            return Point {
                x: current.x - 1,
                y: current.y,
            };
        } else if right.is_some() && [Tile::EW, Tile::NW, Tile::SW, Tile::START].contains(&right.unwrap()) && prev_pos != Pos::Rigth && [Tile::EW, Tile::NE, Tile::SE].contains(&current_tile) {
            return Point {
                x: current.x + 1,
                y: current.y,
            };
        } else {
            panic!("something went wrong. current point: {:?}", current);
        }
    }

    fn get_tile(&self, point: Point) -> Option<Tile> {
        // let tile = self.tiles[point.y][point.x];
        match self.tiles.get(point.y) {
            Some(vec) => {
                match vec.get(point.x) {
                    Some(tile) => return Some(tile.clone()),
                    None => return None,
                }
            }
            None => return None,
        };
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut start: Point = Point::new();
        for (i, line) in value.lines().enumerate() {
            tiles.push(Vec::new());
            for (j, ch) in line.chars().enumerate() {
                let tile = Tile::from(ch);
                if tile == Tile::START {
                    start = Point { x: j, y: i };
                }
                tiles[i].push(tile);
            }
        }
        Map { tiles, start }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\n\nStart: {:?}\n\n", self.start))?;
        for line in &self.tiles {
            f.write_fmt(format_args!("{:?}\n", line))?;
        }
        std::fmt::Result::Ok(())
    }
}
