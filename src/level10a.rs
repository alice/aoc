use std::cmp::min;
use std::io::{self, Read};

fn level10() {
    let num_elements: usize = 256;
    let mut knot: Vec<i32> = Vec::new();
    for i in 0..num_elements {
        knot.push(i as i32);
    }
    let extra_lengths = [17, 31, 73, 47, 23];
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lengths = Vec::new();
    lengths.extend_from_slice(input.trim().as_bytes());
    lengths.extend_from_slice(&extra_lengths);
    let lengths = lengths;

    let mut current_idx: usize = 0;
    let mut skip: usize = 0;

    for _ in 0..64 {
        let lengths_copy = lengths.clone();
        for length in lengths_copy {
            let mut to_reverse = Vec::new();
            {
                let virtual_subslice_end = current_idx + length as usize;
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
            current_idx = (current_idx + length as usize + skip) % num_elements;
            skip += 1;
        }
    }
    let mut hex: String = String::new();
    for chunk in knot.chunks(16) {
        println!("chunk: {:?}", chunk);
        let xor = chunk.iter().fold(0, |acc, &x| acc ^ x);
        println!("xor: {:?}", xor);
        hex.push_str(format!("{:02x}", xor).as_str());
    }
    assert_eq!(hex.len(), 32);
    println!("{:?}", hex);

}

fn main() {
    level10();
}
