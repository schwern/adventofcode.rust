use std::fs::File;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;

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
}

fn checksum( sheet : Vec<Vec<i32>> ) -> i32 {
    let mut sum = 0;
    for row in sheet {
        sum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    return sum;
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
    
    println!("{}", checksum(sheet));
}
