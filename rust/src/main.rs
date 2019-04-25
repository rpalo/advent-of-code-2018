// #[macro_use] extern crate lazy_static;
// extern crate regex;

use std::fs;
use std::io::prelude::*;

mod day18;

fn main() {
    let text = fs::read_to_string("data/day18.txt").unwrap();
    let mut logging = day18::Logging::new(&text);

    for i in 0..1700 { 
        logging = logging.tick();
        if i % 100 == 0 {
            println!("{}: {}", i, logging.resource_value());
        }
    }
    println!("{}", logging.resource_value());
}
