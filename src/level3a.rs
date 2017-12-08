use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Coords {
    fn mv(&mut self, dir: Dir) {
        use Dir::{Up, Left, Down, Right};
        match dir {
            Up => self.y = self.y + 1,
            Left => self.x = self.x - 1,
            Down => self.y = self.y - 1,
            Right => self.x = self.x + 1,
        }
    }
}

fn sum_neighbours(coords: Coords, grid: &HashMap<Coords, i32>) -> i32 {
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            let neighbour_coords = Coords {
                x: (coords.x + i),
                y: (coords.y + j),
            };
            if neighbour_coords == coords {
                continue;
            }
            if !grid.contains_key(&neighbour_coords) {
                continue;
            }
            sum += grid.get(&neighbour_coords).unwrap();
        }
    }
    return sum;
}

fn level3a() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    let number: i32 = input.trim().parse().unwrap();

    let mut grid: HashMap<Coords, i32> = HashMap::new();
    let mut current_coords = Coords { x: 0, y: 0 };
    use Dir::{Up, Left, Down, Right};
    let mut dir_idx = 0;
    let dirs = [Right, Up, Left, Down];
    grid.insert(current_coords.clone(), 1);
    for side_length in 1..10 {
        println!(
            "side_length {}, current_coords: {:?}",
            side_length,
            current_coords
        );
        for _ in 0..2 {
            for _ in 0..side_length {
                current_coords.mv(dirs[dir_idx]);
                let val = sum_neighbours(current_coords, &grid);
                println!("val for {:?} -> {}", current_coords, val);
                if val > number {
                    return;
                }
                grid.insert(current_coords.clone(), val);
            }
            dir_idx = (dir_idx + 1) % 4;
        }
    }
}

fn main() {
    level3a();
}
