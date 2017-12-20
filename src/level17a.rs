fn level17() {
    let mut buffer: Vec<usize> = Vec::new();
    buffer.push(0);
    let mut pos: usize = 0;
    let steps = 328;
    for i in 1..50000000 {
        pos = (pos + steps) % i;
        pos += 1;
        if pos == 1 {
            buffer.insert(pos, i);
            println!("modified buffer, i={}", i);
        }
    }
    println!("{}", buffer[1]);
}

fn main() {
    level17();
}
