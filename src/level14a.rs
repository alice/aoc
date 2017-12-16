mod knot;

fn count_bits(n: u64) -> u64 {
    let mut count = 0;
    for i in 0..64 {
        count += (n >> i) & 1;
    }
    return count;
}

fn to_bit_array(first: u64, second: u64, result: &mut [u8; 128]) {
    for i in 0..64 {
        let bit: u8 = (second >> i & 1) as u8;
        result[127 - i] = bit;
    }
    for i in 0..64 {
        let bit: u8 = (first >> i & 1) as u8;
        result[63 - i] = bit;
    }
}

fn zero_contiguous(grid: &mut [[u8; 128]; 128], start: (usize, usize)) -> bool {
    let row = start.0;
    let col = start.1;
    if grid[row][col] == 0 {
        return false;
    }
    grid[row][col] = 0;
    if row > 0 {
        zero_contiguous(grid, (row - 1, col));
    }
    if row < 127 {
        zero_contiguous(grid, (row + 1, col));
    }
    if col > 0 {
        zero_contiguous(grid, (row, col - 1));
    }
    if col < 127 {
        zero_contiguous(grid, (row, col + 1));
    }
    return true;
}

fn level14() {
    let key_str = "amgozmfv";
    let mut bit_count = 0;
    let mut grid: [[u8; 128]; 128] = [[0; 128]; 128];
    for i in 0..128 {
        let mut line_string: String = String::new();
        line_string.push_str(key_str.clone());
        line_string.push_str("-");
        line_string.push_str(i.to_string().as_str());

        let line_knot: String = knot::knot(line_string.as_str());
        let (first_str, second_str) = line_knot.split_at(16);
        let first = u64::from_str_radix(first_str, 16).unwrap();
        let second = u64::from_str_radix(second_str, 16).unwrap();
        bit_count += count_bits(first);
        bit_count += count_bits(second);

        to_bit_array(first, second, &mut grid[i]);
    }
    println!("bit_count: {}", bit_count);
    let mut region_count = 0;
    for i in 0..128 as usize {
        for j in 0..128 as usize {
            if zero_contiguous(&mut grid, (i, j)) {
                region_count += 1;
            }
        }
    }
    println!("region_count: {}", region_count);
}

fn main() {
    level14();
}
