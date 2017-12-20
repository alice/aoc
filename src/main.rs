extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn level18() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n");
    let line_re = Regex::new(r"(\w{3}) (\w) ?(-?\d+)?").unwrap();
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max = 0;
    for line in lines {
        let captures = line_re.captures(line).unwrap();
        let instr: & = captures[1];

        let register: String = &captures[2].to_owned();

        let param: i32 = 0;
        if (captures.get(3) != None) {
            let param: i32 = captures[3].parse().unwrap();
        }

        let other_reg_name: String = captures[4].to_owned();
        let other_reg_value: i32;
        {
            other_reg_value = registers.entry(other_reg_name).or_insert(0).clone();
        }

        let cmp: &str = &captures[5];
        let cmp_param: i32 = captures[6].parse().unwrap();

        let should_exec = match cmp {
            "<=" => other_reg_value <= cmp_param,
            ">=" => other_reg_value >= cmp_param,
            "==" => other_reg_value == cmp_param,
            "<" => other_reg_value < cmp_param,
            ">" => other_reg_value > cmp_param,
            "!=" => other_reg_value != cmp_param,
            _ => false,
        };

        if should_exec {
            let reg: &mut i32 = registers.entry(reg_name).or_insert(0);
            match instr {
                "inc" => *reg += param,
                "dec" => *reg -= param,
                _ => (),
            }
            if *reg > max {
                max = reg.clone();
            }
        }
    }
    println!("{:?}", registers);
    println!("max: {:?}", registers.values().max().unwrap());
    println!("all time max: {}", max);
}

fn main() {
    level18();
}
