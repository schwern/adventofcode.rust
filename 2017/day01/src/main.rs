use std::io::prelude::*;
use std::fs::File;
use std::env;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    
    #[test]
    fn test_captcha_part_one() {
        let mut tests : HashMap<_,u32> = HashMap::new();
        tests.insert("1122", 3);
        tests.insert("1111", 4);
        tests.insert("1234", 0);
        tests.insert("91212129", 9);
        
        for (arg, &want) in &tests {
            assert_eq!( captcha_part_one(arg), want );
        }
    }
    
    #[test]
    fn test_captcha_part_two() {
        let mut tests : HashMap<_,u32> = HashMap::new();
        tests.insert("1212", 6);
        tests.insert("1221", 0);
        tests.insert("123425", 4);
        tests.insert("123123", 12);
        tests.insert("12131415", 4);
        
        for (arg, &want) in &tests {
            assert_eq!( captcha_part_two(arg), want );
        }
    }
}

fn captcha_part_one(c: &str) -> u32 {
    return captcha(c, 1);
}

fn captcha_part_two(c: &str) -> u32 {
    return captcha(c, c.len()/2);
}

fn captcha(captcha: &str, offset: usize) -> u32 {
    if captcha.len() < 2 {
        return 0;
    }

    let nums : Vec<u32> = captcha
                            .chars()
                            .map( |c| c.to_digit(10).expect("A digit") )
                            .collect();
    
    let len = nums.len();
    let mut sum : u32 = 0;
    for (i,&num) in nums.iter().enumerate() {
        let next = nums[(i+offset)%len];
        if num == next {
            sum += num;
        }
    }
    
    return sum;
}

fn main() {
    let filename = env::args().nth(1).expect("Need a filename");
    let mut fh = File::open(filename).expect("File not found");
    let mut input = String::new();
    fh.read_to_string(&mut input)
        .expect("Error reading file");
    
    println!("{}", captcha_part_two(&input));
}
