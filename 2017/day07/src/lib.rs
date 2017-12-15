#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from() {
        let mut np = NodeParser::new();
        
        assert_eq!(
            *np.try_from("pbga (66)").unwrap(),
            Node::new( "pbga", Some(66), None )
        );
    }
    
    #[test]
    fn test_from_with_children() {
        let mut np = NodeParser::new();
        
        let want_children = vec![
            Node::new("ktlj", None, None),
            Node::new("cntj", None, None),
            Node::new("xhth", None, None),
        ];
        assert_eq!(
            np.try_from("fwft (72) -> ktlj, cntj, xhth").unwrap(),
            &Node::new("fwft", Some(72), Some(want_children) )
        );
    }
    
    #[test]
    fn test_has_children() {
        assert!( !Node::new("foo", None, None ).has_children() );
        assert!( !Node::new("foo", None, Some(vec![])).has_children() );
        assert!(
            Node::new(
                "foo", None,
                Some(vec![ Node::new("bar", None, None) ])
            ).has_children()
        );
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NodeParser<'a> {
    map: HashMap<String,&'a Node>,
    pub root: Option<&'a Node>,
}

impl<'a> NodeParser<'a> {
    pub fn new() -> NodeParser<'a> {
        return NodeParser{
            map:   HashMap::new(),
            root:       None
        };
    }
    
    pub fn try_from( &mut self, from: &str ) -> Result<&'a Node,ParseNodeError> {
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
            
        let children = match caps.name("children") {
            Some(children_from) => Some(
                self.parse_children(children_from.as_str())?
            ),
            None                => None,
        };
        
        let mut node = self.map
            .entry(String::from(name))
            .or_insert( &Node::new( name, None, None ) );

        node.weight = Some(weight);
        node.children = children;
        
        return Ok(node);
    }
    
    fn parse_children( &self, from: &str ) -> Result<Vec<Node>,ParseNodeError> {
        let mut children = Vec::new();
        for child_name in from.split(", ") {
            children.push( Node::new( child_name, None, None ) );
        }
        
        return Ok(children);
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub name: String,
    pub weight: Option<usize>,
    pub children: Option<Vec<Node>>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ParseNodeError {
    Invalid,
    MissingName,
    MissingWeight,
}

impl Node {
    pub fn new( name: &str, weight: Option<usize>, children: Option<Vec<Node>> ) -> Node {
        return Node{
            name: String::from(name),
            weight: weight,
            children: children,
        };
    }
    
    pub fn has_children(&self) -> bool {
        return match self.children {
            Some(ref children)  => !children.is_empty(),
            None                => false
        };
    }
}
