use std::char;
use std::collections::HashMap;
use std::io::{self, Read};

trait Dance {
    fn spin(&mut self, x: usize);
    fn exchange(&mut self, a: usize, b: usize);
    fn partner(&mut self, a: char, b: char);
}

impl Dance for String {
    fn spin(&mut self, x: usize) {
        let end = self.len();
        let split = self.split_off(end - x);
        self.insert_str(0, split.as_str());
    }

    fn exchange(&mut self, a: usize, b: usize) {
        unsafe {
            self.as_mut_vec().swap(a, b);
        }
    }

    fn partner(&mut self, a: char, b: char) {
        let mut a_idx: usize = 0;
        let mut b_idx: usize = 0;
        let mut found_a = false;
        let mut found_b = false;

        for (i, char) in self.chars().enumerate() {
            if u32::from(char) == u32::from(a) {
                found_a = true;
                a_idx = i;
            }
            if u32::from(char) == u32::from(b) {
                found_b = true;
                b_idx = i;
            }
            if found_a && found_b {
                break;
            }
        }
        self.exchange(a_idx, b_idx);
    }
}

enum Move {
    Spin(usize),
    Exchange { a: usize, b: usize },
    Partner { a: char, b: char },
}

fn level16() {
    let num_programs = 16;
    let mut programs: String = String::new();
    for i in 0..num_programs {
        programs.push(char::from_u32(u32::from('a') + i).unwrap());
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let moves = input.trim().split(",");
    let mut compiled_moves: Vec<Move> = Vec::new();
    for mut mv in moves.map(|m| m.to_string()) {
        let t = mv.remove(0);
        if t == 's' {
            let x: usize = mv.parse().unwrap();
            compiled_moves.push(Move::Spin(x));
        } else if t == 'x' {
            let mut split = mv.split("/");
            let a: usize = split.next().unwrap().parse().unwrap();
            let b: usize = split.next().unwrap().parse().unwrap();
            compiled_moves.push(Move::Exchange { a, b });
        } else if t == 'p' {
            let mut split = mv.split("/");
            let a = split.next().unwrap().chars().next().unwrap();
            let b = split.next().unwrap().chars().next().unwrap();
            compiled_moves.push(Move::Partner { a, b });
        }
    }
    let num_moves = compiled_moves.len();
    let compiled_moves = compiled_moves;
    let mut seen: HashMap<String, u32> = HashMap::new();
    let mut period = 0;
    for round in 1..1000000000 {
        for i in 0..num_moves {
            match compiled_moves[i] {
                Move::Spin(x) => programs.spin(x),
                Move::Exchange { a, b } => programs.exchange(a, b),
                Move::Partner { a, b } => programs.partner(a, b),
            }
        }
        if seen.contains_key(&programs) {
            period = round - seen.get(&programs).unwrap();
            println!(
                "{:?} repeated after {} rounds",
                programs,
                round - seen.get(&programs).unwrap()
            );
            break;
        } else {
            seen.insert(programs.clone(), round);
        }
        if round % 10000 == 0 {
            println!("done {}: {:?}", round, programs);
        }
    }

    let index = 1000000000 % period;
    for (programs, i) in seen {
        if i == index {
            println!("after 1 billion runs: {:?}", programs);
        }
    }
}

fn main() {
    level16();
}
