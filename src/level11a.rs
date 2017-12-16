use std::io;


#[derive(Clone, Debug, PartialEq)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn mv(&mut self, dir: &str) {
        self.x = match dir {
            "ne" | "se" => self.x + 0.5,
            "nw" | "sw" => self.x - 0.5,
            _ => self.x,
        };
        self.y = match dir {
            "n" => self.y + 1.0,
            "s" => self.y - 1.0,
            "ne" | "nw" => self.y + 0.5,
            "se" | "sw" => self.y - 0.5,
            _ => panic!("bad direction: {:?}", dir),
        };
    }

    fn mv_origin(&mut self, canonical_steps: &mut Vec<String>) {
        let dir: &str;
        if self.x > 0.0 {
            if self.y > 0.0 {
                dir = "sw";
            } else {
                dir = "nw";
            }
        } else if self.x < 0.0 {
            if self.y > 0.0 {
                dir = "se";
            } else {
                dir = "ne";
            }
        } else {
            if self.y > 0.0 {
                dir = "s";
            } else {
                dir = "n";
            }
        }
        canonical_steps.push(dir.to_owned());
        self.mv(dir);
    }

    fn distance_from_origin(&self) -> usize {
        let origin: Pos = Pos { x: 0.0, y: 0.0 };
        let mut pos = self.clone();
        let mut canonical_steps: Vec<String> = Vec::new();
        while pos != origin {
            pos.mv_origin(&mut canonical_steps);
        }
        return canonical_steps.len();
    }
}

impl Eq for Pos {}

fn level11() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let directions: Vec<String> = input.trim().split(",").map(|x| x.to_owned()).collect();

    let mut pos: Pos = Pos { x: 0.0, y: 0.0 };
    let mut max_steps: usize = 0;
    for dir in directions {
        pos.mv(&dir.as_str());
        let distance_from_origin = pos.distance_from_origin();
        if distance_from_origin > max_steps {
            max_steps = distance_from_origin;
        }
    }
    println!(
        "current distance_from_origin: {}, max distance_from_origin: {}, pos: {:?}",
        pos.distance_from_origin(),
        max_steps,
        pos
    );
}

fn main() {
    level11();
}
