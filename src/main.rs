extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

#[derive(Default)]
struct Program<'a> {
    name: &'a str,
    weight: i32,
    supporting: HashSet<&'a Program<'a>>,
    supported_by: Option<&'a Program<'a>>,
}

impl<'a> PartialEq for Program<'a> {
    fn eq(&self, other: &Program<'a>) -> bool {
        return self.name == other.name;
    }
}

impl<'a> Eq for Program<'a> {}

impl<'a> Hash for Program<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn level7() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n");
    let re = Regex::new(r"(\w+) \((\d+)\)(:? -> )?(:?(\w+)(:?, )?)*").unwrap();
    for line in lines {
        println!("line: {:?}", line);
        let captures = re.captures(line).unwrap();
        let name: &str = &captures[1];
        let weight: i32 = captures[2].parse().unwrap();
        let program: Program = Program {
            name,
            weight,
            ..Default::default()
        };
        println!("captures: {:?}", captures);
        println!("name: {:?}, weight: {:?}, supports: {:?}", name, weight, 0)
    }
}

fn main() {
    level7();
}
