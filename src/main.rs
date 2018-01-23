#[macro_use]
extern crate ndarray;

use ndarray::Ix2;
use ndarray::{Array2, ArrayView, arr2};

const ITERATIONS: usize = 5;
const START_SIZE: usize = 3;
const MAX_SIZE: usize = ITERATIONS + START_SIZE;
const START_GRID: [[bool; 3]; 3] = [
    [false, true, false],
    [false, false, true],
    [true, true, true],
];

fn print_grid(grid: &ArrayView<bool, Ix2>) {
    for row in grid.genrows() {
        let row_str = row.iter()
            .map(|b| if *b { "#" } else { "." })
            .collect::<Vec<&str>>()
            .join("");
        println!("{}", row_str);
    }
}

fn level21() {
    let mut grid: Array2<bool> = Array2::<bool>::default((MAX_SIZE, MAX_SIZE));
    let mut current_size: usize = START_SIZE;
    {
        let mut current_view = grid.slice_mut(s![0..current_size, 0..current_size]);
        current_view.assign(&arr2(&START_GRID));
    }
    {
        let mut current_view = grid.slice(s![0..current_size, 0..current_size]);
        print_grid(&current_view);
        println!("");
        print_grid(&(current_view.reversed_axes()));
        print_grid(&(current_view.reversed_axes()));
    }
}

fn main() {
    level21();
}
