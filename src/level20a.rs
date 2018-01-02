extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::default::Default;
use std::fmt::Debug;
use std::fmt::{Error, Formatter};
use std::io::{self, Read};
use std::ops::AddAssign;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Triplet {
    x: i64,
    y: i64,
    z: i64,
}

impl Triplet {
    fn magnitude(&self) -> f32 {
        ((self.x as f32).powi(2) + (self.y as f32).powi(2) + (self.z as f32).powi(2)).sqrt()
    }
}

impl AddAssign for Triplet {
    fn add_assign(&mut self, rhs: Triplet) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Debug for Triplet {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}

impl FromStr for Triplet {
    type Err = ParseError;
    // in the form x,y,z
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xyz: Vec<&str> = s.split(",").collect();
        let x: i64 = xyz[0].parse().unwrap();
        let y: i64 = xyz[1].parse().unwrap();
        let z: i64 = xyz[2].parse().unwrap();
        Ok(Triplet { x, y, z })
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
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

impl Debug for Particle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "p={:?}, v={:?}, a={:?}", self.p, self.v, self.a)
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
    let mut particles: Vec<Particle> = Vec::new();
    for (_i, line) in lines.enumerate() {
        println!("line: {:?}", line);
        let particle: Particle = line.parse().unwrap();
        particles.push(particle);
    }


    for _tick in 0..100 {
        if _tick % 10000 == 0 {
            println!("tick {}", _tick);
        }
        let mut collided: HashSet<usize> = HashSet::new();
        let mut positions: HashMap<Triplet, usize> = HashMap::new();
        for i in 0..particles.len() {
            if collided.contains(&i) {
                continue;
            }
            let particle = &particles[i];
            if positions.contains_key(&particle.p) {
                let other: usize = positions.get(&particle.p).unwrap().clone();
                println!("at tick {}, {} collided with {}", _tick, i, other);
                collided.insert(i);
                collided.insert(other);
                continue;
            }

            positions.insert(particle.p.clone(), i);
        }
        let mut sorted_collided: Vec<&usize> = collided.iter().collect();
        sorted_collided.sort();
        sorted_collided.reverse();
        for i in sorted_collided {
            particles.remove(*i);
        }
        for i in 0..particles.len() {
            let particle = particles.get_mut(i).unwrap();
            //println!("particle before: {:?}", particle);
            particle.v += particle.a.clone();
            particle.p += particle.v.clone();
            //println!("particle after: {:?}", particle);
        }
    }
    println!("remaining: {}", particles.len());
}

fn main() {
    level20();
}
