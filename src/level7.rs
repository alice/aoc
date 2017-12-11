extern crate regex;

use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};

#[derive(Debug, Default)]
struct Program<'a> {
    name: String,
    weight: i32,
    supports: HashSet<String>,
    supported_by: Cell<Option<&'a Program<'a>>>,
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
    let line_re = Regex::new(r"(\w+) \((\d+)\)(?: -> )?((?:\w+(?:, )?)+)?").unwrap();
    let mut programs: HashMap<String, Program> = HashMap::new();
    for line in lines {
        let captures = line_re.captures(line).unwrap();
        let name = captures[1].to_owned();
        let name_key = name.clone();
        let weight: i32 = captures[2].parse().unwrap();
        let mut program: Program = Program {
            name,
            weight,
            ..Default::default()
        };
        if captures.get(3) != None {
            let supports = captures.get(3).unwrap().as_str();
            for supports_name in supports.split(", ") {
                program.supports.insert(String::from(supports_name));
            }
        }
        programs.insert(name_key, program);
    }
    for program in programs.values() {
        for supports_name in &program.supports {
            if !programs.contains_key(supports_name) {
                continue;
            }
            let supports_program = &programs.get(supports_name).expect(
                format!(
                    "Could not find program with name {:?}",
                    supports_name
                ).as_str(),
            );
            supports_program.supported_by.set(Some(program));
        }
    }
    for program in programs.values() {
        if program.supported_by.get() == None {
            println!("Not supported by anything: {:?}", program.name);
        }
    }
}

fn main() {
    level7();
}
