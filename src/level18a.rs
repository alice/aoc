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

fn run(instrs: Vec<Instr>, tx: Sender<i64>, rx: Receiver<i64>, pid: i64) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    registers.insert("p".to_owned(), pid);
    let mut next_instr = 0;
    let mut total_sent = 0;
    let id: String = thread::current().name().unwrap().into();
    let mut iters = 0;
    while next_instr < instrs.len() && iters < 1000000 {
        iters += 1;
        // println!("{:} next_instr = {}", id, next_instr);
        let instr: &Instr = &instrs[next_instr];
        // println!("{:} {:?}", id, instr);
        match instr {
            &Instr::Snd { ref x, x_val } => {
                let value: i64 = get_value!(x, x_val, registers);
                let sent = tx.send(value);
                if sent.is_err() {
                    println!("channel closed, stopping {}", id);
                    break;
                }
                total_sent += 1;
                println!("{} sent {} - total {} sent", id, value, total_sent);
            }
            &Instr::Rcv { ref x } => {
                let try_received = rx.try_recv();
                let received: i64;
                if try_received.is_err() {
                    println!("{} Waiting to receive", id);
                    received = rx.recv().unwrap();
                } else {
                    received = try_received.unwrap();
                }
                {
                    {
                        let register = get_register!(x, registers);
                        *register = received;
                    }
                    /* println!(
                        "{} Received {}, assigned to {:?} ({})",
                        id,
                        received,
                        x,
                        registers.get(x).unwrap()
                    ); */
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
                    // println!("{:} jumping to {}", id, next_instr);
                    continue;
                }
            }
            &Instr::Set {
                ref x,
                ref y,
                y_val,
            } => {
                {
                    let value = get_value!(y, y_val, registers);
                    let register = get_register!(x, registers);
                    // println!("{:} setting {:?} to {}", id, x, value);
                    *register = value;
                }
                // println!("{:} {:?} is now {}", id, x, registers.get(x).unwrap());
            }
            &Instr::Add {
                ref x,
                ref y,
                y_val,
            } => {
                {
                    let value = get_value!(y, y_val, registers);
                    let register = get_register!(x, registers);
                    // println!("{:} adding {} to {:?} ({})", id, value, x, register);
                    *register += value;
                }
                // println!("{:} {:?} is now {}", id, x, registers.get(x).unwrap());
            }
            &Instr::Mul {
                ref x,
                ref y,
                y_val,
            } => {
                {
                    let value = get_value!(y, y_val, registers);
                    let register = get_register!(x, registers);
                    // println!("{:} multiplying {:?} ({}) by {}", id, x, register, value);
                    *register *= value;
                }
                // println!("{:} {:?} is now {}", id, x, registers.get(x).unwrap());
            }
            &Instr::Mod {
                ref x,
                ref y,
                y_val,
            } => {
                {
                    let value = get_value!(y, y_val, registers);
                    let register = get_register!(x, registers);
                    // println!("{:} modding {:?} ({}) by {}", id, x, register, value);
                    *register %= value;
                }
                // println!("{:} {:?} is now {}", id, x, registers.get(x).unwrap());
            }
        };

        next_instr += 1;
    }
    println!("done executing {:}, closing channel", id);
}

fn level18() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("input: {:?}", input);
    let lines: Vec<String> = input.trim().split("\n").map(|s| s.to_owned()).collect();
    println!("lines: {:?}", lines);
    let instrs = compile(lines);
    let instrs_copy = instrs.clone();
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    println!("spawning thread 1");
    let handle1 = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || {
            println!("running 1");
            run(instrs, tx1, rx2, 0);
        })
        .unwrap();
    println!("spawning thread 2");
    let handle2 = thread::Builder::new()
        .name(
            "                                        thread2".to_string(),
        )
        .spawn(move || {
            println!("running 2");
            run(instrs_copy, tx2, rx1, 1);
        })
        .unwrap();
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    level18();
}
