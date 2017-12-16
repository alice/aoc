use std::cmp::min;
use std::io::{self, Read};

fn level10() {
    let num_elements: usize = 256;
    let mut knot: Vec<i32> = Vec::new();
    for i in 0..num_elements {
        knot.push(i as i32);
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lengths: Vec<usize> = input
        .trim()
        .split(",")
        .map(|x| x.parse())
        .map(|y| y.unwrap())
        .collect();

    let mut current_idx: usize = 0;
    let mut skip: usize = 0;

    for length in lengths {
        let mut to_reverse = Vec::new();
        {
            let virtual_subslice_end = current_idx + length;
            let mut subslice_end = min(virtual_subslice_end, num_elements);
            to_reverse.extend_from_slice(knot.get(current_idx..subslice_end).unwrap());
            subslice_end = virtual_subslice_end - subslice_end;
            if subslice_end > 0 {
                to_reverse.extend_from_slice(knot.get(0..subslice_end).unwrap());
            }
        }
        to_reverse.reverse();
        for (i, v) in to_reverse.iter().enumerate() {
            knot[((current_idx + i) % num_elements) as usize] = v.clone();
        }
        current_idx = (current_idx + length + skip) % num_elements;
        skip += 1;
    }
    println!(
        "Product of {} and {}: {}",
        knot[0],
        knot[1],
        knot[0] * knot[1]
    );

}

fn main() {
    level10();
}
