use core::panic;
use std::{cmp::Ordering, fs, io};

#[derive(Eq, Debug)]
struct Dice {
    r: u32,
    g: u32,
    b: u32,
}

impl PartialOrd for Dice {
    fn ge(&self, other: &Self) -> bool {
        self.r >= other.r && self.g >= other.g && self.b >= other.b
    }
    fn gt(&self, other: &Self) -> bool {
        self.r > other.r && self.g > other.g && self.b > other.b
    }
    fn le(&self, other: &Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
    fn lt(&self, other: &Self) -> bool {
        self.r < other.r && self.g < other.g && self.b < other.b
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dice {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Dice {
    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    dice_sets: Vec<Dice>
}

impl Game {
    fn possible_with(&self, dice: &Dice) -> bool {
        self.dice_sets
            .iter()
            .all(|d| d <= dice)
    }

    fn min(&self) -> Dice {
        Dice {
            r: self.dice_sets
                .iter()
                .map(|d| d.r)
                .max()
                .unwrap(),
            g: self.dice_sets
                .iter()
                .map(|d| d.g)
                .max()
                .unwrap(),
            b: self.dice_sets
                .iter()
                .map(|d| d.b)
                .max()
                .unwrap()
        }
    }
}

fn parse_game(line: &str) -> Result<Game, io::Error> {
    // get game id
    let game_id = line.
        split_once(":").unwrap().0
        .split_once(" ").unwrap().1
        .parse::<u32>()
        .unwrap();

    // skip "Game n: "
    let start_index = line.find(":").unwrap();
    let line = &line[start_index + 2..];

    // get dice sets of the game
    let dice_vec: Vec<Dice> = line
        .split("; ")
        .into_iter()
        .map(|dice_str: &str| {
            let mut dice: Dice = Dice{ r: 0, g: 0, b: 0 };
            for color_str in dice_str.split(", ") {
                match color_str.split_once(" ").unwrap() {
                    (n, "red") => dice.r = n.parse::<u32>().unwrap(),
                    (n, "green") => dice.g = n.parse::<u32>().unwrap(),
                    (n, "blue") => dice.b = n.parse::<u32>().unwrap(),
                    _ => panic!("couldn't parse dice color"),
                }
            }
            dice
        }).collect::<Vec<Dice>>();

    let game = Game{ id: game_id, dice_sets: dice_vec };
    Ok(game)
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    static DICE_BAG: Dice = Dice {
        r: 12,
        g: 13,
        b: 14,
    };

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        games.push(parse_game(line)?)
    }

    // task 1
    let mut sum = 0;
    for game in &games {
        if game.possible_with(&DICE_BAG) {
            sum += game.id;
        }
    }
    println!("task 1: {sum}");

    // task 2
    let mut sum = 0;
    for game in games {
        let min_dice = game.min();
        let power = min_dice.power();
        sum += power;
    }
    println!("task 2: {sum}");


    return Ok(());
}
