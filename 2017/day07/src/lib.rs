#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from() {
        assert_eq!(
            Node::try_from("pbga (66)").unwrap(),
            Node::new( "pbga", 66 )
        );
    }
    
    #[test]
    fn test_from_with_children() {
        assert_eq!(
            Node::try_from("fwft (72) -> ktlj, cntj, xhth").unwrap(),
            Node::new("fwft", 72)
        );
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    name: String,
    weight: usize,
    children: Vec<Node>
}

#[derive(PartialEq, Clone, Debug)]
pub enum ParseNodeError {
    Invalid,
    MissingName,
    MissingWeight,
}

impl Node {
    pub fn new( name: &str, weight: usize ) -> Node {
        return Node{
            name: String::from(name),
            weight: weight,
            children: Vec::new()
        };
    }
    
    pub fn has_children(&self) -> bool {
        return !self.children.is_empty();
    }

    pub fn try_from( from: &str ) -> Result<Node,ParseNodeError> {
        lazy_static! {
            static ref PARSE_RE: Regex = Regex::new(r"(?x)
                # name
                (?P<name> [[:alpha:]]+ )
                \s+
                # digit
                \(
                    (?P<weight> [[:digit:]]+ )
                \)
                # children (parsed later)
                (?: \s+ -> \s+ (?P<children>.+) )?
            ").unwrap();
        }
        
        let caps = PARSE_RE.captures(from)
            .ok_or(ParseNodeError::Invalid)?;
        let name = caps.name("name")
            .ok_or(ParseNodeError::MissingName)?
            .as_str();
        let weight: usize = caps.name("weight")
            .ok_or(ParseNodeError::MissingWeight)?
            .as_str()
            .parse()
            .unwrap();
        let node = Node::new(
            name,
            weight,
        );
        
        return Ok(node);
    }
}
