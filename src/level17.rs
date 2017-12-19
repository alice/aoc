fn level17() {
    let mut buffer: Vec<u32> = Vec::new();
    buffer.push(0);
    let mut pos: usize = 0;
    let steps = 328;
    for i in 1..2018 {
        pos = (pos + steps) % buffer.len();
        pos += 1;
        buffer.insert(pos, i);
    }
    println!("{}", buffer[pos + 1]);
}

fn main() {
    level17();
}
