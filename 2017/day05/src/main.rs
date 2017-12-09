use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_num_jumps() {
        assert_eq!( num_jumps( vec![0,3,0,1,-3] ), 10 );
    }
}

fn num_jumps( mut offsets : Vec<i32> ) -> usize {
    let mut pos : i32 = 0;
    let mut steps :usize = 0;
    loop {
        match offsets.get_mut(pos as usize) {
            Some(jump)  => {
                steps += 1;
                
                pos += *jump;
                if *jump >= 3 {
                    *jump -= 1;
                }
                else {
                    *jump += 1;
                }
            },
            None        => return steps,
        }
    }
}

fn main() {
    let file = env::args().nth(1).expect("input file");
    let fh = File::open(file).expect("can't open file");
    let reader = BufReader::new(fh);
    
    let mut jumps : Vec<i32> = Vec::new();
    for line in reader.lines() {
        jumps.push(line.unwrap().parse().expect("an integer"));
    }
    
    println!("{}", num_jumps(jumps));
}
