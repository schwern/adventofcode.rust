extern crate day03;

#[macro_use]
extern crate clap;

use day03::Ring;

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
    println!("{}", Ring::manhattan_distance(input));
}

fn part_two(input : u32) {
    println!("input: {}", input);
}
