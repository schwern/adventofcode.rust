use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_num_cycles_before_repeat() {
        let banks = vec![0,2,7,0];
        assert_eq!( num_cycles_before_repeat( banks ), (4,5) );
    }
    
    #[test]
    fn test_next_cycle() {
        let mut banks = vec![0,2,7,0];
        let wants = vec![
            vec![2,4,1,2],
            vec![3,1,2,3],
            vec![0,2,3,4],
            vec![1,3,4,1],
            vec![2,4,1,2],
        ];
        
        for want in wants {
            next_cycle(&mut banks);
            assert_eq!( banks, want );
        }
    }
}

fn num_cycles_before_repeat( _banks : Vec<i32> ) -> (u32,u32) {
    let mut banks = _banks.clone();
    let mut seen = HashMap::new();
    
    let mut num_cycles : u32 = 1;
    let first_cycle;
    loop {
        if seen.contains_key(&banks) {
            first_cycle = *seen.get(&banks).unwrap();
            break;
        }
        else {
            seen.insert(banks.clone(), num_cycles );
            next_cycle(&mut banks);
            num_cycles += 1;
        }        
    }
    
    return ((num_cycles - first_cycle), seen.len() as u32);
}

// Can't use enumerate().max_by(),
// we must have the first max if there's a tie.
fn max_index( banks : &Vec<i32> ) -> usize {
    let mut max_index : usize = 0;
    let mut max = i32::min_value();
    for (pos,val) in banks.iter().enumerate() {
        if *val > max {
            max = *val;
            max_index = pos;
        }
    }

    return max_index;
}

fn next_cycle( banks : &mut Vec<i32> ) {
    let mut pos = max_index(banks);
    let mut distribute = banks[pos];

    banks[pos] = 0;

    loop {
        if distribute <= 0 {
            return;
        }
        
        pos += 1;
        pos %= banks.len();
        
        banks[pos] += 1;
        distribute -= 1;
    }
}

fn main() {
    let file = env::args().nth(1).expect("input file");
    let mut fh = File::open(file).expect("can't open file");
    let mut input = String::new();
    fh.read_to_string(&mut input).unwrap();
    let banks :Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("integer"))
        .collect();
    
    let(cycles,_) = num_cycles_before_repeat(banks);
    println!("{}", cycles);
}
