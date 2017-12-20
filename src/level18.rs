extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn level18() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.trim().split("\n").collect();
    let line_re = Regex::new(r"(\w{3}) (\w) ?(-?\d+|\w)?").unwrap();
    let num_re = Regex::new(r"-?\d+").unwrap();
    let register_re = Regex::new(r"\w").unwrap();
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut last_snd = -1;
    let mut next_line = 0;

    while next_line < lines.len() {
        let line = lines[next_line];
        println!("{:?}", line);
        let captures = line_re.captures(line).unwrap();
        let instr: &str = &captures[1];

        let mut param: i64 = 0;
        if captures.get(3) != None {
            let p: &str = &captures[3];
            if num_re.is_match(p) {
                param = p.parse().unwrap();
            } else if register_re.is_match(p) {
                let name: String = p.to_owned();
                param = registers[&name].clone();
            }
        }

        let register_name = captures[2].to_owned();
        let register: &mut i64;
        let reg_value: i64;
        {
            register = registers.entry(register_name.clone()).or_insert(0);
            reg_value = register.clone();
        }


        match instr {
            "snd" => {
                last_snd = reg_value;
                println!("last_snd = {}", last_snd);
            }
            "set" => {
                println!("setting {:?} to {}", register_name, param);
                *register = param
            }
            "add" => {
                println!("adding {} to {}", param, reg_value);
                *register += param
            }
            "mul" => {
                println!("multiplying {} by {}", register, param);
                *register *= param;
            }
            "mod" => {
                println!("modding {} by {}", reg_value, param);
                *register %= param
            }
            "rcv" => {
                if reg_value > 0 {
                    break;
                }
            }
            "jgz" => {
                if reg_value > 0 {
                    next_line = (next_line as i64 + param) as usize;
                    println!("jumping to {}", next_line);
                    continue;
                }
            }
            &_ => panic!("Wrong instr found: {:?}", instr),
        };

        next_line += 1;
    }
    println!("{}", last_snd);
}

fn main() {
    level18();
}
