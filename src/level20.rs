extern crate regex;

use regex::Regex;
use std::default::Default;
use std::io::{self, Read};
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Default)]
struct Triplet {
    x: i32,
    y: i32,
    z: i32,
}

impl Triplet {
    fn magnitude(&self) -> f32 {
        ((self.x as f32).powi(2) + (self.y as f32).powi(2) + (self.z as f32).powi(2)).sqrt()
    }
}

impl FromStr for Triplet {
    type Err = ParseError;
    // in the form x,y,z
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xyz: Vec<&str> = s.split(",").collect();
        let x: i32 = xyz[0].parse().unwrap();
        let y: i32 = xyz[1].parse().unwrap();
        let z: i32 = xyz[2].parse().unwrap();
        Ok(Triplet { x, y, z })
    }
}

#[derive(Debug)]
struct Particle {
    p: Triplet,
    v: Triplet,
    a: Triplet,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            p: Default::default(),
            v: Default::default(),
            a: Default::default(),
        }
    }
}

impl FromStr for Particle {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Particle = Particle::new();
        let assignment_re = Regex::new(r"(\w)=<(.*)>").unwrap();
        for assignment in s.split(", ") {
            let captures = assignment_re.captures(assignment).unwrap();
            let field: &str = &captures[1];
            let triple: Triplet = captures[2].parse().unwrap();
            match field {
                "p" => result.p = triple,
                "v" => result.v = triple,
                "a" => result.a = triple,
                _ => panic!("what's a {:?}", field),
            }
        }
        Ok(result)
    }
}

fn level20() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n");
    let mut min_accel: f32 = 0.0;
    let mut slowest: usize = 0;
    let mut first = true;
    for (i, line) in lines.enumerate() {
        println!("line: {:?}", line);
        let particle: Particle = line.parse().unwrap();
        if first || particle.a.magnitude() < min_accel {
            min_accel = particle.a.magnitude();
            slowest = i;
            first = false;
        }
    }
    println!("slowest: {} ({})", slowest, min_accel);
}

fn main() {
    level20();
}
