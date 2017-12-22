extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};
use std::iter::Iterator;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

#[derive(Debug, Clone)]
enum Instr {
    Snd { x: String, x_val: Option<i64> },
    Rcv { x: String },
    Jgz {
        x: String,
        x_val: Option<i64>,
        y: String,
        y_val: Option<i64>,
    },
    Set {
        x: String,
        y: String,
        y_val: Option<i64>,
    },
    Add {
        x: String,
        y: String,
        y_val: Option<i64>,
    },
    Mul {
        x: String,
        y: String,
        y_val: Option<i64>,
    },
    Mod {
        x: String,
        y: String,
        y_val: Option<i64>,
    },
}

fn compile(lines: Vec<String>) -> Vec<Instr> {
    let line_re = Regex::new(r"(\w{3}) (\w) ?(-?\d+|\w)?").unwrap();
    let num_re = Regex::new(r"-?\d+").unwrap();

    let mut instrs = Vec::<Instr>::new();
    for line in lines {
        let captures = line_re.captures(line.as_str()).unwrap();
        let instr: &str = &captures[1];
        let x: &str = &captures[2].to_owned();
        let x_val: Option<i64> = if num_re.is_match(x) {
            Some(x.parse().unwrap())
        } else {
            None
        };
        let mut y: Option<&str> = None;
        let mut y_val: Option<i64> = None;
        if captures.get(3) != None {
            y = Some(&captures[3]);
            if num_re.is_match(y.unwrap()) {
                y_val = Some(y.unwrap().parse().unwrap());
            }
        }

        match instr {
            "snd" => {
                instrs.push(Instr::Snd {
                    x: x.to_owned(),
                    x_val,
                });
            }
            "rcv" => {
                instrs.push(Instr::Rcv { x: x.to_owned() });
            }
            "jgz" => {
                instrs.push(Instr::Jgz {
                    x: x.to_owned(),
                    x_val,
                    y: y.unwrap().to_owned(),
                    y_val,
                });
            }
            "set" => {
                instrs.push(Instr::Set {
                    x: x.to_owned(),
                    y: y.unwrap().to_owned(),
                    y_val,
                });
            }
            "add" => {
                instrs.push(Instr::Add {
                    x: x.to_owned(),
                    y: y.unwrap().to_owned(),
                    y_val,
                });
            }
            "mul" => {
                instrs.push(Instr::Mul {
                    x: x.to_owned(),
                    y: y.unwrap().to_owned(),
                    y_val,
                });
            }
            "mod" => {
                instrs.push(Instr::Mod {
                    x: x.to_owned(),
                    y: y.unwrap().to_owned(),
                    y_val,
                });
            }
            &_ => panic!("Wrong instr found: {:?}", instr),
        };
    }
    return instrs;
}

macro_rules! get_value {
    ($name:expr, $value:expr, $registers:expr) => {{
    match $value {
        Some(val) => val,
        None => *$registers.entry($name.clone()).or_insert(0),
    }
    }}
}

macro_rules! get_register {
    ($name:expr, $registers:expr) => {{
         $registers.entry($name.clone()).or_insert(0)
    }}
}

fn run(instrs: Vec<Instr>, tx: Sender<i64>, rx: Receiver<i64>) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut next_instr = 0;
    let mut total_sent = 0;
    let id: String = thread::current().name().unwrap().into();
    while next_instr < instrs.len() {
        let instr: &Instr = &instrs[next_instr];

        match instr {
            &Instr::Snd { ref x, x_val } => {
                let value: i64 = get_value!(x, x_val, registers);
                let sent = tx.send(value);
                if sent.is_err() {
                    println!("channel closed, stopping {:?}", id);
                    break;
                }
                total_sent += 1;
                println!("sent {} from {:?} - total {} sent", value, id, total_sent);
            }
            &Instr::Rcv { ref x } => {
                let received = rx.recv();
                {
                    let register = get_register!(x, registers);
                    *register = received.unwrap();
                }
            }
            &Instr::Jgz {
                ref x,
                x_val,
                ref y,
                y_val,
            } => {
                let value: i64 = get_value!(x, x_val, registers);
                let jump: i64 = get_value!(y, y_val, registers);
                if value > 0 {
                    next_instr = (next_instr as i64 + jump) as usize;
                    //println!("jumping to {}", next_line);
                    continue;
                }
            }
            &Instr::Set {
                ref x,
                ref y,
                y_val,
            } => {
                let value = get_value!(y, y_val, registers);
                let register = get_register!(x, registers);
                //println!("setting {:?} to {}", register_name, param);
                *register = value;
            }
            &Instr::Add {
                ref x,
                ref y,
                y_val,
            } => {
                let value = get_value!(y, y_val, registers);
                let register = get_register!(x, registers);
                //println!("adding {} to {}", param, reg_value);
                *register += value;
            }
            &Instr::Mul {
                ref x,
                ref y,
                y_val,
            } => {
                let value = get_value!(y, y_val, registers);
                let register = get_register!(x, registers);
                //println!("multiplying {} by {}", register, param);
                *register *= value;
            }
            &Instr::Mod {
                ref x,
                ref y,
                y_val,
            } => {
                let value = get_value!(y, y_val, registers);
                let register = get_register!(x, registers);
                //println!("modding {} by {}", reg_value, param);
                *register %= value
            }
        };

        next_instr += 1;
    }
    println!("done executing {:?}, closing channel", id);
}

fn level18() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<String> = input.trim().split("\n").map(|s| s.to_owned()).collect();
    let instrs = compile(lines);
    let instrs_copy = instrs.clone();
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    println!("spawning thread 1");
    let handle1 = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || {
            println!("running 1");
            run(instrs, tx1, rx2);
        })
        .unwrap();
    println!("spawning thread 2");
    let handle2 = thread::Builder::new()
        .name("thread2".to_string())
        .spawn(move || {
            println!("running 2");
            run(instrs_copy, tx2, rx1);
        })
        .unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    level18();
}
