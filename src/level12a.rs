extern crate regex;

use regex::{Regex, Captures};
use std::collections::HashSet;
use std::io;


fn level12() {
    let mut line = String::new();

    let mut programs: Vec<Vec<usize>> = Vec::new();
    let line_re = Regex::new(r"(\d+) <-> ((?:\d+(?:, )?)+)?").unwrap();
    while io::stdin().read_line(&mut line).unwrap() != 0 {
        {
            let captures_opt: Option<Captures> = line_re.captures(line.as_str());
            if captures_opt.is_none() {
                break;
            }
            let captures = captures_opt.unwrap();
            let id: usize = captures[1].parse().unwrap();
            let talks_to: Vec<usize> = captures[2]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            println!("line: {:?}, id: {}, talks_to: {:?}", line, id, talks_to);
            programs.push(talks_to.clone());
            assert_eq!(programs.len(), id + 1);
        }
        line.clear();
    }

    let mut seen: HashSet<usize> = HashSet::new();
    let mut unseen: HashSet<usize> = HashSet::new();
    for i in 0..programs.len() {
        unseen.insert(i);
    }
    let mut groups = 0;

    while unseen.len() > 0 {
        let mut stack: Vec<usize>;
        {
            stack = programs[unseen.iter().next().unwrap().clone()].clone();
        }
        while stack.len() > 0 {
            let program = stack.pop().unwrap();
            if seen.contains(&program) {
                continue;
            }
            seen.insert(program);
            unseen.remove(&program);
            let talks_to = &programs[program];
            stack.extend_from_slice(&talks_to);
        }
        groups += 1;
    }
    println!("groups: {}", groups);

}

fn main() {
    level12();
}
