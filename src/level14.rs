mod knot;

fn count_bits(n: u64) -> u64 {
    let mut count = 0;
    for i in 0..64 {
        count += (n >> i) & 1;
    }
    return count;
}

fn level14() {
    let key_str = "amgozmfv";
    let mut bit_count = 0;
    for i in 0..128 {
        let mut line_string: String = String::new();
        line_string.push_str(key_str.clone());
        line_string.push_str("-");
        line_string.push_str(i.to_string().as_str());

        let line_knot: String = knot::knot(line_string.as_str());
        let (first, second) = line_knot.split_at(16);
        bit_count += count_bits(u64::from_str_radix(first, 16).unwrap());
        bit_count += count_bits(u64::from_str_radix(second, 16).unwrap());
    }
    println!("total bit_count: {}", bit_count);
}

fn main() {
    level14();
}
