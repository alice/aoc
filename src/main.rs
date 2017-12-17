use std::char;
use std::io::{self, Read};

trait Dance {
    fn spin(&mut self, args: &str);
    fn exchange(&mut self, args: &str);
    fn swap(&mut self, a: usize, b: usize);
    fn partner(&mut self, args: &str);
}

impl Dance for String {
    fn spin(&mut self, args: &str) {
        let x: usize = args.parse().unwrap();
        let end = self.len();
        let split = self.split_off(end - x);
        self.insert_str(0, split.as_str());
    }

    fn exchange(&mut self, args: &str) {
        let mut split = args.split("/");
        let a: usize = split.next().unwrap().parse().unwrap();
        let b: usize = split.next().unwrap().parse().unwrap();
        self.swap(a, b);
    }

    fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            self.as_mut_vec().swap(a, b);
        }
    }

    fn partner(&mut self, args: &str) {
        let mut split = args.split("/");
        let a = split.next().unwrap().chars().next().unwrap();
        let b = split.next().unwrap().chars().next().unwrap();
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
        self.swap(a_idx, b_idx);
    }
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
    for mut mv in moves.map(|m| m.to_string()) {
        println!("mv: {:?}", mv);
        let t = mv.remove(0);
        match t {
            's' => programs.spin(mv.as_str()),
            'x' => programs.exchange(mv.as_str()),
            'p' => programs.partner(mv.as_str()),
            _ => panic!("weird type {:?}", t),
        }
    }

    println!("{:?}", programs);
}

fn main() {
    level16();
}
