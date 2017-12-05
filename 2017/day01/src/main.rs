use std::io::prelude::*;
use std::fs::File;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_captcha() {
        assert_eq!( captcha("1122"), 3 );
        assert_eq!( captcha("1111"), 4 );
        assert_eq!( captcha("1234"), 0 );
        assert_eq!( captcha("91212129"), 9 );
    }
}

fn captcha(captcha: &str) -> u32 {
    if captcha.len() < 2 {
        return 0;
    }

    let nums : Vec<u32> = captcha
                            .chars()
                            .map( |c| c.to_digit(10).expect("A digit") )
                            .collect();
    
    let len = nums.len();
    let mut sum : u32 = 0;
    for (i, &num) in nums.iter().enumerate() {
        let next = nums[(i+1)%len];
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
    
    println!("{}", captcha(&input));
}
