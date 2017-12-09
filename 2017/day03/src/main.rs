extern crate day03;

#[macro_use]
extern crate clap;

use day03::ring;
use day03::spiral;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_first_larger_in_spiral() {
        assert_eq!( first_larger_in_spiral(3), 4 );
        assert_eq!( first_larger_in_spiral(747), 806 );
        assert_eq!( first_larger_in_spiral(360), 362 );
    }
}

fn parse_u32( input : &str ) -> u32 {
    return input.parse().expect("positive integer");
}

fn main() {
    let app_matches = clap_app!(day03 =>
        (version: "1.0.0")
        (author: "Michael G Schwern")
        (about: "Advent Of Code 2017 Day 3")
        (@subcommand one =>
            (about: "Day 3 Part 1")
            (@arg square: +required)
        )
        (@subcommand two =>
            (about: "Day 3 Part 2")
            (@arg square: +required)
        )
    ).get_matches();
    
    match app_matches.subcommand() {
        ("one", Some(sub))   => {
            let input :u32 = parse_u32( sub.value_of("square").unwrap() );
            part_one(input);
        },
        ("two", Some(sub))   => {
            let input :u32 = parse_u32( sub.value_of("square").unwrap() );
            part_two(input);
        },
        _                    => println!("{}", app_matches.usage()),
    }
}

fn part_one(input : u32) {
    println!("{}", ring::Ring::manhattan_distance(input));
}

fn first_larger_in_spiral( input: u32 ) -> u32 {
    let mut spiral = spiral::Spiral::new();
    let mut grid : HashMap<spiral::Point,u32> = HashMap::new();

    // Start with 1 at the center.
    grid.insert(spiral.next().unwrap(), 1);

    let zero :u32 = 0;
    let mut val : u32 = 0;
    for point in spiral {
        val = 0;
        for neighbor in point.neighbors() {
            let nval = grid.get(&neighbor).unwrap_or(&zero);
            val += nval;
        }
        
        grid.insert(point, val);
        
        if val > input {
            break;
        }
    }
    
    return val;
}

fn part_two(input : u32) {
    println!("{}", first_larger_in_spiral(input));
}
