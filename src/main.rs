fn level15() {
    let mut a: u64 = 516;
    let mut b: u64 = 190;

    let a_multiplier: u64 = 16807;
    let b_multiplier: u64 = 48271;
    let divisor: u64 = 2147483647;

    let mut a_values: Vec<u64> = Vec::new();
    let mut b_values: Vec<u64> = Vec::new();

    let mut count = 0;
    let mut pair = 0;
    loop {
        a = (a * a_multiplier) % divisor;
        if a % 4 == 0 {
            a_values.push(a);
        }
        b = (b * b_multiplier) % divisor;
        if b % 8 == 0 {
            b_values.push(b);
        }
        if a_values.len() > 0 && b_values.len() > 0 {
            pair += 1;
            let next_pair = (a_values.remove(0), b_values.remove(0));
            if next_pair.0 & 0xffff == next_pair.1 & 0xffff {
                count += 1;
                if count % 10 == 0 {
                    println!("count {} pairs {}", count, pair);
                }
            }
        }
        if pair >= 5000000 {
            break;
        }
    }
    println!("count: {}", count);
}

fn main() {
    level15();
}
