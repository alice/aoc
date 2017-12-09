use std::io::{self, Read};

trait Anagrams {
    fn is_anagram_of(&self, other: &Self) -> bool;
}

impl Anagrams for str {
    fn is_anagram_of(&self, other: &str) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut self_chars: Vec<char> = self.chars().collect();
        self_chars.sort();
        let mut other_chars: Vec<char> = other.chars().collect();
        other_chars.sort();

        for char_pair in self_chars.iter().zip(other_chars.iter()) {
            if char_pair.0 != char_pair.1 {
                return false;
            }
        }

        return true;
    }
}

fn valid_passphrase(passphrase: &str) -> bool {
    let words: Vec<&str> = passphrase.split_whitespace().collect();

    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if words[i].is_anagram_of(words[j]) {
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
