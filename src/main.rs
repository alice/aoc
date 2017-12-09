use std::collections::HashMap;
use std::io::{self, Read};

fn level6() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut banks: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse().expect(
                format!("could not parse {:?}", x).as_str(),
            )
        })
        .collect();
    let mut steps = 0;
    let mut seen: HashMap<Vec<i32>, i32> = HashMap::new();
    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), steps);
        let mut max = 0;
        let mut max_at = 0;
        banks.iter().enumerate().for_each(
            |(i, &bank)| if bank > max {
                max = bank;
                max_at = i;
            },
        );
        banks[max_at] = 0;
        let mut remaining = max;
        let mut i = (max_at + 1) % banks.len();
        while remaining > 0 {
            banks[i] += 1;
            remaining -= 1;
            i = (i + 1) % banks.len();
        }
        steps += 1;
    }

    println!("loop size: {}", steps - seen.get(&banks).unwrap());
}

fn main() {
    level6();
}
