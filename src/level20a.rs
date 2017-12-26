extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::default::Default;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Default, Eq, Hash, PartialEq)]
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

    pub fn intersect(&self, other: &Particle) -> Option<(f32, f32, f32)> {
        let mut intersection = (0.0, 0.0, 0.0);
        let mut found = false;
        for plane in vec!["x", "y", "z"] {
            // for each plane, convert position + velocity + accel to quadratic
            let q1 = self.quadratic(plane);
            let q2 = other.quadratic(plane);
            // then compute partial intersection using find_intersection
            let partial_intersection: Option<f32> = find_intersect(q1, q2);
            if partial_intersection.is_none() {
                continue;
            }
            found = true;
            println!(
                "partial_intersection({}): {}",
                plane,
                partial_intersection.unwrap()
            );
            match plane {
                "x" => intersection.0 = partial_intersection.unwrap(),
                "y" => intersection.1 = partial_intersection.unwrap(),
                "z" => intersection.2 = partial_intersection.unwrap(),
                _ => (),
            }
        }
        if (!found) {
            return None;
        }
        return Some(intersection);
    }

    fn quadratic(&self, plane: &str) -> (f32, f32, f32) {
        match plane {
            "x" => return make_quadratic(self.p.x, self.v.x, self.a.x),
            "y" => return make_quadratic(self.p.y, self.v.y, self.a.y),
            "z" => return make_quadratic(self.p.z, self.v.z, self.a.z),
            _ => panic!("not a plane: {}", plane),
        }
    }

    fn time_at_point(&self, (x, y, z): (f32, f32, f32)) -> i32 {
        println!("time_at_point({}, {}, {})", x, y, z);
        let time_x = find_positive_root((
            self.a.x as f32 / 2.0,
            self.v.x as f32,
            self.p.x as f32 - x,
        ));
        let time_y = find_positive_root((
            self.a.y as f32 / 2.0,
            self.v.y as f32,
            self.p.y as f32 - y,
        ));
        let time_z = find_positive_root((
            self.a.z as f32 / 2.0,
            self.v.z as f32,
            self.p.z as f32 - z,
        ));
        println!(
            "time_x: {:?}, time_y: {:?}, time_z: {:?}",
            time_x,
            time_y,
            time_z
        );
        return 0;
    }
}

fn make_quadratic(p: i32, v: i32, a: i32) -> (f32, f32, f32) {
    return (p as f32, v as f32, (a / 2) as f32);
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

fn find_intersect((a1, b1, c1): (f32, f32, f32), (a2, b2, c2): (f32, f32, f32)) -> Option<f32> {
    let a: f32 = a1 - a2;
    let b: f32 = b1 - b2;
    let c: f32 = c1 - c2;

    return find_positive_root((a, b, c));
}

fn find_positive_root((a, b, c): (f32, f32, f32)) -> Option<f32> {
    println!("find_positive_root ({}, {}, {})", a, b, c);
    if a == 0.0 {
        return None;
    }
    let discriminant: f32 = b.powi(2) - 4.0 * a * c;
    println!("discriminant: {}", discriminant);

    if discriminant < 0.0 {
        return None;
    }

    let root_delta: f32 = discriminant.sqrt();
    println!("root_delta: {}", root_delta);

    return Some((-b + root_delta) / (a * 2.0));
}


fn level20() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n");
    let mut particles: Vec<Particle> = Vec::new();
    for (i, line) in lines.enumerate() {
        println!("line: {:?}", line);
        let particle: Particle = line.parse().unwrap();
        particles.push(particle);
    }

    let mut collided: HashSet<usize> = HashSet::new();
    for i in 0..particles.len() {
        if collided.contains(&i) {
            continue;
        }
        let particle_i = &particles[i];
        for j in 0..particles.len() {
            if collided.contains(&j) || i == j {
                continue;
            }
            let particle_j = &particles[j];
            let opt_intersection = particle_i.intersect(particle_j);
            if opt_intersection.is_none() {
                continue;
            }
            let intersection = opt_intersection.unwrap();
            println!("intersection: {:?}", intersection);
            // work out whether both particles hit intersection at the same time
            if particle_i.time_at_point(intersection) == particle_j.time_at_point(intersection) {
                collided.insert(i);
                collided.insert(j);
            }
        }
    }
}

fn main() {
    level20();
}
