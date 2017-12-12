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
    cumulative_weight: Cell<i32>,
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

fn level7a() {
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
    let mut root_option: Option<&Program> = None;
    for program in programs.values() {
        if program.supported_by.get() != None {
            continue;
        }
        root_option = Some(program);
        break;
    }
    let root = root_option.unwrap();

    let mut stack: Vec<&Program> = Vec::new();
    let mut queue: Vec<&Program> = Vec::new();
    queue.insert(0, root);
    while let Some(program) = queue.pop() {
        println!("pushing {:?}", program.name);
        stack.push(program);
        for name in &program.supports {
            queue.insert(0, &programs.get(name).unwrap());
        }
    }

    while let Some(program) = stack.pop() {
        program.cumulative_weight.set(
            program.cumulative_weight.get() +
                program.weight,
        );
        if program.supported_by.get() == None {
            continue;
        }
        let supported_by: &Program = program.supported_by.get().unwrap();
        supported_by.cumulative_weight.set(
            supported_by.cumulative_weight.get() +
                program.cumulative_weight.get(),
        );
    }

    let mut next: Option<&Program> = Some(root);
    while next != None {
        let program = next.unwrap();
        let mut weights: HashMap<i32, &Program> = HashMap::new();
        let mut wrong_weight: Option<&Program> = None;
        let mut right_weight: Option<i32> = None;
        // find the weight which is different
        for name in &program.supports {
            let other = programs.get(name).unwrap();
            let weight: i32 = other.cumulative_weight.get();
            println!("other: {:?}, cumulative_weight: {:?}", name, weight);

            if weights.contains_key(&weight) || right_weight == None {
                right_weight = Some(weight);
            } else {
                wrong_weight = Some(other);
            }

            weights.insert(weight, other);
        }
        if wrong_weight == None {
            println!("Found wrong program: {:?}", program.name);
            let siblings = &program.supported_by.get().unwrap().supports;
            let mut right_weight: Option<i32> = None;
            for name in siblings {
                let sibling = programs.get(name).unwrap();
                if sibling.cumulative_weight.get() == program.cumulative_weight.get() {
                    continue;
                }
                right_weight = Some(sibling.cumulative_weight.get());
                break;
            }
            let difference = right_weight.unwrap() - program.cumulative_weight.get();
            let should_weigh = program.weight + difference;
            println!(
                "Weighs {}, Should weigh {}, difference is {}",
                program.weight,
                should_weigh,
                difference
            );
            return;
        }
        next = Some(wrong_weight.unwrap());
    }
}

fn main() {
    level7a();
}
