#![allow(unused)]
use itertools::Itertools;
use std::{collections::HashMap, fs, io::Error, fmt::DebugStruct};

fn build_mappings(lines: Vec<&str>) -> Vec<Vec<(usize, usize, usize)>> {
    let mut maps: Vec<Vec<(usize, usize, usize)>> = Vec::new();

    for line in lines[2..].iter() {
        if line.contains(":") {
            let mut map: Vec<(usize, usize, usize)> = Vec::new();
            maps.push(map);
            continue;
        } else if *line == "" {
            continue;
        }

        let (dest, source, range) = line
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
        maps.last_mut().unwrap().push((source, dest, range));
    }

    for mut map in maps.iter_mut() {
        map.sort_by_key(|(s, _, _)| *s);
    }


    maps
}


fn traverse_maps(maps: &Vec<Vec<(usize, usize, usize)>>, seed: usize) -> usize {
    let mut curr_location = seed;
    for map in maps.iter() {
        for (source, dest, range) in map {
            if (curr_location >= *source) && (curr_location < *source + range) {
                curr_location = dest + curr_location - *source;
                break;
            }
        }
    }

    curr_location
}


fn parse_seed_ranges(maps: Vec<Vec<(usize, usize, usize)>>, seed_ranges: Vec<usize>) -> usize {
    let mut min_location: usize = std::usize::MAX;
    for i in 0..(seed_ranges.len() / 2) {
        let (start, range) = (seed_ranges[i * 2], seed_ranges[i * 2 + 1]);
        for seed in start..(start + range) {
            // todo compute location directly here and check against minimum
            let location = traverse_maps(&maps, seed);
            if location < min_location {
                min_location = location;
            }
        }
        println!("seed {start} - {range} done");
    }

    min_location
}


fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let seeds: Vec<usize> = lines[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let maps = build_mappings(lines);

    // task 1
    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds.clone() {
        locations.push(traverse_maps(&maps, seed))
    }
    let task_1 = locations.iter().min().unwrap();
    println!("task 1: {task_1}");

    // task 2
    let mut locations: Vec<usize> = Vec::new();
    let task_2 = parse_seed_ranges(maps, seeds);
    println!("task 2: {task_2}");

    Ok(())
}
