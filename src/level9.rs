extern crate regex;

use std::io::{self, Read};
use std::str::Chars;

#[derive(Default, Debug)]
struct State {
    stack_size: i32,
    count: i32,
}

impl State {
    fn consume_characters(&mut self, chars: &mut Chars) {
        let next = chars.next();
        if next == None {
            return;
        }
        match next.unwrap() {
            '{' => self.consume_group(chars),
            '<' => self.consume_garbage(chars),
            _ => panic!("Unexpected character: {:?}", next.unwrap()),
        }
    }

    fn consume_group(&mut self, chars: &mut Chars) {
        self.stack_size += 1;
        let next = chars.next();
        match next.unwrap() {
            '<' => self.consume_garbage(chars),
            '{' => self.consume_group(chars),
            '}' => self.end_group(chars),
            _ => panic!("Unexpected character: {:?}", next.unwrap()),
        }
    }

    fn end_group(&mut self, chars: &mut Chars) {
        self.count += self.stack_size;
        self.stack_size -= 1;
        if self.stack_size < 0 {
            panic!("Unexpected end of group");
        }
        self.end(chars);
    }

    fn end(&mut self, chars: &mut Chars) {
        let next = chars.next();
        if next == None {
            if self.stack_size == 0 {
                return;
            }
            panic!("Unexpected end of input");
        }
        match next.unwrap() {
            ',' => self.consume_characters(chars),
            '}' => self.end_group(chars),
            _ => panic!("Unexpected character: {:?}", next.unwrap()),
        }
    }

    fn consume_garbage(&mut self, chars: &mut Chars) {
        let mut next = chars.next();
        while next != None {
            next = match next.unwrap() {
                '!' => chars.nth(1),
                '>' => None,
                _ => chars.next(),
            }
        }
        self.end(chars);
    }
}

fn level9() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n");
    for line in lines {
        let mut state: State = State { ..Default::default() };
        let mut chars: Chars = line.chars();
        state.consume_characters(&mut chars);
        println!("score: {}", state.count);
    }
}

fn main() {
    level9();
}
