use std::io;

fn level1() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    let mut digits: Vec<&str> = input.split("").collect();
    digits.remove(0);
    digits.pop();
    digits.pop();

    let mut dupdigits: Vec<&str> = Vec::new();
    let len = digits.len();
    let jump = len / 2; // previously 1
    for i in 0..len {
        let j = (i + jump) % len;
        if digits[i] == digits[j] {
            dupdigits.push(digits[i]);
        }
    }
    let mut result: u32 = 0;
    for digit in dupdigits {
        let num: u32 = digit.parse().unwrap();
        result += num;
    }
    println!("result: {}", result);
}

fn main() {
    level1();
}
