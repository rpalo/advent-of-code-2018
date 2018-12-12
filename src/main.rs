// #[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;

mod day11;

fn main() {
    
    // let mut input = File::open("data/day10.txt").expect("File not found.");

    // let mut contents = String::new();
    // input.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut grid = day11::Grid::new(8772);
    println!("{:?}", grid.best_cell_sized());
    //sky.display(20000);
    
}
