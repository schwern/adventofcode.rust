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
    
    let first = captcha.chars().nth(0).expect("At least one digit");
    let mut iter = captcha.chars().peekable();
    let mut sum : u32 = 0;
    
    loop {
        let curr_item = iter.next();

        let matched = match( curr_item, iter.peek() ) {
            (Some(curr),Some(&next))   => curr == next,
            (Some(curr),None)          => curr == first,
            (None,_)                   => break,
        };
        
        if matched {
            let num = curr_item.expect("An item").to_digit(10).expect("Not a digit");
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
