use std::collections::HashSet;
use std::error::Error;

#[derive(Clone)]
struct Cell {
    x: i32,
    y: i32,
}
impl Cell {
    fn in_range(&self, n: i32, m: i32) -> bool {
        0 <= self.x && self.x < m && 0 <= self.y && self.y < n
    }
}

#[derive(Clone)]
struct CandidateData {
    string: String,
    current_cell: Cell,
    used_cell: Vec<Cell>,
}
impl CandidateData {
    fn is_used(&self, cell: &Cell) -> bool {
        for used in self.used_cell.iter() {
            if used.x == cell.x && used.y == cell.y {
                return true;
            }
        }
        return false;
    }
}

struct PrimeString {
    list: Vec<String>,
}
impl PrimeString {
    fn new(max: i32) -> PrimeString {
        let mut list = vec![];
        'outer: for i in 2..max {
            let i_f64 = i as f64;
            let sqrt_i = i_f64.sqrt();
            let end = (sqrt_i + 1.0) as i32;
            for j in 2..end {
                if i % j == 0 {
                    continue 'outer;
                }
            }
            list.push(i.to_string());
        }
        PrimeString { list }
    }
    fn ends_with(&self, key: &str) -> bool {
        for prime in self.list.iter() {
            if prime.ends_with(key) {
                return true;
            }
        }
        return false;
    }
    fn is_prime(&self, key: &String) -> bool {
        for prime in self.list.iter() {
            if prime == key {
                return true;
            }
        }
        return false;
    }
}

fn get_cell_char(s: &[String], cell: &Cell) -> char {
    let chars: Vec<char> = s[cell.y as usize].chars().collect();
    chars[cell.x as usize]
}

fn insert_prime_if(
    prime_string: &PrimeString,
    key: &String,
    mut candidate_list: HashSet<String>,
) -> HashSet<String> {
    if prime_string.is_prime(key) {
        candidate_list.insert(key.clone());
    }
    candidate_list
}

fn serch_route(
    n: i32,
    m: i32,
    s: &[String],
    prime_string: &PrimeString,
    mut candidate_data_vec: Vec<CandidateData>,
    mut candidate_list: HashSet<String>,
) -> HashSet<String> {
    while !candidate_data_vec.is_empty() {
        if let Some(data) = candidate_data_vec.pop() {
            if data.string.len() == 5 {
                continue;
            }
            for add_index in [
                Cell { x: 1, y: 0 },
                Cell { x: 0, y: 1 },
                Cell { x: -1, y: 0 },
                Cell { x: 0, y: -1 },
            ] {
                let current_cell = Cell {
                    x: data.current_cell.x + add_index.x,
                    y: data.current_cell.y + add_index.y,
                };
                if !current_cell.in_range(n, m) || data.is_used(&current_cell) {
                    continue;
                }
                let c = get_cell_char(s, &current_cell);
                if c == '.' {
                    continue;
                }
                if data.string.len() == 4 && c == '0' {
                    continue;
                }
                let string = String::from(c) + data.string.as_str();
                if prime_string.ends_with(&string) {
                    candidate_list = insert_prime_if(prime_string, &string, candidate_list);
                    let mut used_cell = data.used_cell.clone();
                    used_cell.push(current_cell.clone());
                    candidate_data_vec.push(CandidateData {
                        string,
                        current_cell,
                        used_cell,
                    });
                }
            }
        }
    }
    candidate_list
}

fn get_max_score(candidate_list: &HashSet<String>) -> i32 {
    let mut max_score = 0;
    for candidate in candidate_list {
        let base_score: i32 = candidate.parse().expect("parse error");
        let mut score = base_score;
        if candidate.find('0').is_some() {
            score += base_score;
        }
        if *candidate == candidate.chars().rev().collect::<String>() {
            score += base_score;
        }
        if max_score < score {
            max_score = score;
        }
    }
    max_score
}

fn main() -> Result<(), Box<dyn Error>> {
    proconio::input! {
      n:i32,
      m:i32,
      s: [String; n],
    };

    let prime_string = PrimeString::new(100000);

    let mut candidate_list = HashSet::new();
    for y in 0..n {
        for x in 0..m {
            let current_cell = Cell { x, y };
            let c = get_cell_char(&s, &current_cell);
            if c == '.' {
                continue;
            }
            candidate_list = insert_prime_if(&prime_string, &String::from(c), candidate_list);
            if c == '2' || c == '5' || c == '0' {
                continue;
            }
            let candidate_data_vec: Vec<CandidateData> = vec![CandidateData {
                string: String::from(c),
                current_cell: current_cell.clone(),
                used_cell: vec![current_cell],
            }];
            candidate_list =
                serch_route(n, m, &s, &prime_string, candidate_data_vec, candidate_list);
        }
    }

    println!("{}", get_max_score(&candidate_list));

    Ok(())
}
