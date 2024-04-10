use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn solve(input_string: &str) -> Option<String> {
    let split_space: Vec<&str> = input_string.split(' ').collect();

    if !(2..=6).contains(&split_space.len()) {
        return None;
    }

    let mut keyword: HashMap<i32, String> = HashMap::new();
    for string in split_space.iter().take(split_space.len() - 1) {
        let split_colon: Vec<&str> = string.split(':').collect();
        if split_colon.len() != 2 {
            return None;
        }
        if split_colon[0].starts_with('0') {
            return None;
        }

        let num: i32 = match split_colon[0].parse() {
            Ok(n) => n,
            Err(_e) => {
                return None;
            }
        };
        if !(1..=100).contains(&num) {
            return None;
        }

        let keyword_len = split_colon[1].len();
        if !(1..=50).contains(&keyword_len) {
            return None;
        }

        if keyword.contains_key(&num) {
            return None;
        }
        keyword.insert(num, split_colon[1].to_string());
    }

    if split_space[split_space.len() - 1].starts_with('0') {
        return None;
    }

    let num: i32 = match split_space[split_space.len() - 1].parse() {
        Ok(n) => n,
        Err(_e) => {
            return None;
        }
    };
    if !(1..=10000000).contains(&num) {
        return None;
    }
    let mut max_num = 0;
    for key in keyword.keys() {
        if *key > max_num && num % key == 0 {
            max_num = *key;
        }
    }
    if max_num == 0 {
        return Some(num.to_string());
    } else {
        return Some(keyword[&max_num].clone());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read from stdin"))
        .collect();

    match solve(&lines[0]) {
        Some(s) => println!("{}", s),
        None => println!("invalid input"),
    };

    Ok(())
}
