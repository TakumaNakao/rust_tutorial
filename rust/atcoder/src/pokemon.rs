fn main() {
    println!("hello");
}

use fxhash::FxHashMap;
use fxhash::FxHashSet;

struct LocateData {
    sum: i64,
    types: FxHashMap<i64, i64>,
}

pub fn create_solution(
    _step: i64,
    _n: i64,
    _m: i64,
    q: i64,
    query1: Vec<i64>,
    query2: Vec<i64>,
    query3: Vec<i64>,
) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    let mut discover_list: FxHashMap<i64, LocateData> = FxHashMap::default();
    let mut discover_types: FxHashSet<i64> = FxHashSet::default();
    for i in 0..q {
        if query3[i as usize] != -1 {
            let x = query1[i as usize];
            let y = query2[i as usize];
            let z = query3[i as usize];

            let locate = discover_list.entry(x).or_insert(LocateData {
                sum: 0,
                types: FxHashMap::default(),
            });
            locate.sum += z;
            *locate.types.entry(y).or_insert(0) += z;
            discover_types.insert(y);
        } else {
            let a = query1[i as usize];
            let b = query2[i as usize];

            if b == 0 {
                result.push(discover_list.len() as i64);
                continue;
            }
            if !discover_types.contains(&a) {
                result.push(0);
                continue;
            }

            let mut count = 0;
            for locate in discover_list.values() {
                if let Some(target) = locate.types.get(&a) {
                    if target * 100 >= b * locate.sum {
                        count += 1;
                    }
                }
            }
            result.push(count);
        }
    }
    result
}
