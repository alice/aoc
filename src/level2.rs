use std::io::{self, Read};

fn level2() {
    let mut rows = String::new();

    io::stdin().read_to_string(&mut rows).expect(
        "Failed to read row",
    );

    let mut checksum = 0;
    for row in rows.split("\n") {
        let numbers: Vec<i32> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut found_checksum = false;
        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i == j {
                    continue;
                }
                let x = numbers[i];
                let y = numbers[j];
                if x % y != 0 {
                    continue;
                }
                checksum += x / y;
                found_checksum = true;
                break;
            }
            if found_checksum {
                break;
            }
        }
    }
    println!("{}", checksum);
}

fn main() {
    level2();
}
