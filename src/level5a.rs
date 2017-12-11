use std::io::{self, Read};
use std::fmt::Write;

fn output_jumps(jumps: &Vec<i32>, pos: usize) {
    let mut output = String::new();
    for i in 0..jumps.len() {
        if i > 0 {
            write!(&mut output, " ").unwrap();
        }
        if i == pos {
            write!(&mut output, "({})", jumps[i]).unwrap();
        } else {
            write!(&mut output, "{}", jumps[i]).unwrap();
        }
    }
    println!("{:?}", output);
}

fn level5() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut jumps: Vec<i32> = input
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut i = 0;
    let mut steps = 0;
    while i < jumps.len() {
        // output_jumps(&jumps, i);
        let jump = jumps[i];
        if jump >= 3 {
            jumps[i] -= 1;
        } else {
            jumps[i] += 1;
        }
        if jump < 0 && jump.abs() as usize > i {
            break;
        }
        i = (i as i32 + jump) as usize;
        steps += 1;
    }
    // output_jumps(&jumps, i);
    println!("{} steps", steps);
}

fn main() {
    level5();
}
