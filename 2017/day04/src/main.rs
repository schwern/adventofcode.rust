#[macro_use]
extern crate maplit;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_valid_password() {
        let tests = hashmap!{
            "aa bb cc dd ee" => true,
            "aa bb cc dd aa" => false,
            "aa bb cc dd aaa" => true,
        };
        for (arg,want) in tests {
            assert_eq!( is_valid_password(arg), want );
        }
    }
}

fn is_valid_password( password : &str ) -> bool {
    let mut words : HashMap<&str,usize> = hashmap!{};
    for word in password.split(" ") {
        let count = words.entry(word).or_insert(0);
        *count += 1;
        
        if *count > 1 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    let file = env::args().nth(1).expect("input file");
    let fh = File::open(file).expect("can't open file");
    let reader = BufReader::new(fh);
    
    let mut num_valid = 0;
    for line in reader.lines() {
        if is_valid_password( &line.unwrap() ) {
            num_valid += 1;
        }
    }
    
    println!("{}", num_valid);
}
