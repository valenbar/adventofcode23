use std::{fs, collections::HashMap};
use once_cell::sync::Lazy;

static BOX_MAP: Lazy<HashMap<String, u8>> = Lazy::new(|| HashMap::new());

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("src/bin/input.txt").expect("input file error");
    let result = task1(input);
    dbg!(result);
    Ok(())
}

fn task1(input: String) -> usize {
    let results = input
        .split(",")
        .map(|s| hash(s.to_string()))
        .collect::<Vec<u8>>();
    let sum = results.iter().map(|&x| x as usize).sum::<usize>();
    sum
}

fn hash(input: String) -> u8 {
    let mut curr_val: usize = 0;
    for ch in input.chars() {
        curr_val += ch as usize;
        curr_val *= 17;
        curr_val %= 256;
    }

    curr_val.try_into().expect("modulo failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_input() {
        let result = task1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string());
        assert_eq!(result, 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".to_string()), 52);
    }
}
