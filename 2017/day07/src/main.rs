extern crate day07;

use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
    }
}

fn find_bottom_node_name( lines: std::io::Lines<std::io::BufReader<std::fs::File>> ) -> String {
    let mut seen = HashMap::new();

    for line in lines {
        let node = day07::Node::try_from( &line.unwrap() ).unwrap();
        seen.entry( node.name.clone() ).or_insert( false );
        
        if node.has_children() {
            for child in node.children.unwrap() {
                seen.insert( child.name.clone(), true );
            }
        }
    }
    
    return seen.into_iter()
        .filter_map( |(k,v)| if !v { Some(k) } else { None } )
        .next()
        .unwrap();
}

fn main() {
    let file = env::args().nth(1).expect("input file");
    let fh = File::open(file).expect("can't open file");
    let reader = BufReader::new(fh);
    
    println!("{}", find_bottom_node_name( reader.lines() ));
}
