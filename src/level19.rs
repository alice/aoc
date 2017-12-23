use std::io::{self, Read};
use std::clone::Clone;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn update_from(&mut self, other: &Pos) {
        self.row = other.row;
        self.col = other.col;
    }

    fn mv(&mut self, dir: &Dir) {
        use Dir::{Up, Down, Left, Right, Unknown};
        match *dir {
            Up => self.row -= 1,
            Down => self.row += 1,
            Left => self.col -= 1,
            Right => self.col += 1,
            Unknown => (),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

trait Walk {
    fn find_next(&self, pos: &mut Pos, prev: &mut Pos, dir: &mut Dir) -> Option<&char>;
    fn find_neighbour(&self, pos: &mut Pos, prev: &Pos, dir: &mut Dir);
    fn char_at(&self, pos: &Pos) -> Option<&char>;
}

impl Walk for Vec<Vec<char>> {
    fn find_next(&self, pos: &mut Pos, prev: &mut Pos, dir: &mut Dir) -> Option<&char> {
        let mut next: Pos = pos.clone();
        match *dir {
            Dir::Unknown => self.find_neighbour(&mut next, prev, dir),
            _ => next.mv(dir),
        }
        prev.update_from(&pos);
        pos.update_from(&next);

        let chr = self.char_at(pos).unwrap();
        if *chr == ' ' {
            return None;
        }

        return Some(chr);
    }

    fn find_neighbour(&self, pos: &mut Pos, prev: &Pos, dir: &mut Dir) {
        use Dir::{Up, Down, Left, Right};
        let mut chr;
        let mut next = pos.clone();
        for next_dir in [Up, Right, Left, Down].iter() {
            next.update_from(pos);
            next.mv(next_dir);
            if next == *prev {
                continue;
            }
            chr = *self.char_at(&next).unwrap();
            if chr != ' ' {
                *dir = (*next_dir).clone();
                break;
            }
        }
        pos.update_from(&next);
    }

    fn char_at(&self, pos: &Pos) -> Option<&char> {
        let row = self.get(pos.row);
        if row.is_some() {
            return row.unwrap().get(pos.col);
        }
        return None;
    }
}

fn level19() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.split("\n").collect();
    let num_lines = lines.len();
    let line_length = lines[0].len();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in 0..num_lines {
        let chars: Vec<char> = lines[line].chars().collect();
        grid.push(chars);
    }
    let grid = grid;

    // Find starting position
    let mut pos = Pos { row: 0, col: 0 };
    let mut prev = pos.clone();
    loop {
        if pos.col < line_length && grid[pos.row][pos.col] == ' ' {
            pos.col += 1;
            continue;
        }
        break;
    }

    let mut chr_opt: Option<&char> = grid.char_at(&pos);
    let mut dir: Dir = Dir::Down;
    let mut seen: Vec<char> = Vec::new();
    while !chr_opt.is_none() {
        let chr = chr_opt.unwrap();
        if chr.is_alphabetic() {
            seen.push(chr.clone());
        }
        if *chr == '+' {
            dir = Dir::Unknown;
        }
        chr_opt = grid.find_next(&mut pos, &mut prev, &mut dir);
    }
    let seen_str: String = seen.into_iter().collect();
    println!("seen: {:?}", seen_str);
}

fn main() {
    level19();
}
