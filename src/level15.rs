fn level15() {
    let mut a: u64 = 516;
    let mut b: u64 = 190;

    let a_multiplier: u64 = 16807;
    let b_multiplier: u64 = 48271;
    let divisor: u64 = 2147483647;

    let mut count = 0;
    for _ in 0..40000000 {
        a = (a * a_multiplier) % divisor;
        b = (b * b_multiplier) % divisor;
        if a & 0xffff == b & 0xffff {
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn main() {
    level15();
}
