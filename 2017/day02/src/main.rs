extern crate itertools;

use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let sheet = vec![
            vec![5,1,9,5],
            vec![7,5,3],
            vec![2,4,6,8]
        ];
        assert_eq!( checksum(sheet), 18 );
    }
    
    #[test]
    fn test_part_two() {
        let sheet = vec![
            vec![5,9,2,8],
            vec![9,4,7,3],
            vec![3,8,6,5],
        ];
        assert_eq!( checksum2(sheet), 9 );
    }
    
    #[test]
    fn test_checksum2_row() {
        assert_eq!( checksum2_row(vec![5,9,2,8]), 4 );
        assert_eq!( checksum2_row(vec![9,4,7,3]), 3 );
        assert_eq!( checksum2_row(vec![3,8,6,5]), 2 );
    }
}


fn checksum( sheet : Vec<Vec<i32>> ) -> i32 {
    let mut sum = 0;
    for row in sheet {
        sum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    return sum;
}

fn checksum2( sheet : Vec<Vec<i32>> ) -> i32 {
    let mut sum = 0;
    for row in sheet {
        sum += checksum2_row(row);
    }
    return sum;
}

fn checksum2_row( row : Vec<i32> ) -> i32 {
    let combos = Itertools::combinations( row.into_iter(), 2 );
    for combo in combos {
        if combo[0] % combo[1] == 0 {
            return combo[0] / combo[1];
        }
        else if combo[1] % combo[0] == 0 {
            return combo[1] / combo[0];
        }
    }
    
    return 0;
}

fn read_sheet( fh : File ) -> Vec<Vec<i32>> {
    let reader = BufReader::new(fh);
    let mut sheet : Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        sheet.push(
            line.unwrap().split_whitespace().map( |i|
                i.parse().expect("A number")
            ).collect()
        )
    }
    
    return sheet;
}

fn main() {
    let filename = env::args().nth(1).expect("Need a filename");
    let fh = File::open(filename).expect("File not found");

    let sheet = read_sheet(fh);
    
    println!("{}", checksum2(sheet));
}
