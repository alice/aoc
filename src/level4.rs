use std::io::{self, Read};

fn valid_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if words[i] == words[j] {
                return false;
            }
        }
    }
    return true;
}
fn level4() {
    let mut passphrases = String::new();

    io::stdin().read_to_string(&mut passphrases).unwrap();
    let mut num_good = 0;
    for passphrase in passphrases.trim().split("\n") {
        if valid_passphrase(passphrase.trim()) {
            num_good += 1;
        }
    }
    println!("num good: {}", num_good);
}

fn main() {
    level4();
}
