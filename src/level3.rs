use std::io;

fn find_side_length(num: i32) -> i32 {
    let mut i: i32 = 1;
    while i.pow(2) < num {
        i += 2;
    }
    return i;
}

fn level3() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    let number: i32 = input.trim().parse().unwrap();
    let side_length: i32 = find_side_length(number);
    // bottom right is square of side length
    let bottom_right = side_length.pow(2);
    let midpoint = (side_length + 1) / 2;
    let num_sides: i32 = (bottom_right - number) / (side_length - 1);
    let within_side: i32 = (bottom_right - number) % (side_length - 1);
    let mut dx = midpoint - 1;
    let mut dy = midpoint - 1;
    if num_sides == 1 || num_sides == 2 {
        dx = -dx
    }
    if num_sides == 2 || num_sides == 3 {
        dy = -dy;
    }
    if num_sides == 0 {
        dx -= within_side;
    }
    if num_sides == 1 {
        dy -= within_side;
    }
    if num_sides == 2 {
        dx += within_side;
    }
    if num_sides == 3 {
        dy += within_side
    }
    let manhattan_distance = dx.abs() + dy.abs();
    println!("manhattan_distance: {}", manhattan_distance);
}

fn main() {
    level3();
}
