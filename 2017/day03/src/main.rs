extern crate day03;
use std::env;

use day03::Ring;

fn main() {
    let pos : u32 = env::args().nth(1).expect("square position")
                               .parse().expect("positive integer");
    println!("{}", Ring::manhattan_distance(pos));
}
