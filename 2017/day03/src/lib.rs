use std::error::Error;
use std::fmt;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_get_manhattan_distance() {
        let mut tests : HashMap<u32,u32> = HashMap::new();
        tests.insert(1, 0);
        tests.insert(12,3);
        tests.insert(23,2);
        tests.insert(1024,31);
        
        for (arg, want) in tests {
            assert_eq!(Ring::manhattan_distance( arg ), want );
        }
    }
    
    #[test]
    fn test_get_ring() {
        let mut tests : HashMap<u32,Ring> = HashMap::new();
        tests.insert(1, Ring{num:0,start:1,end:1,size:1});
        tests.insert(2, Ring{num:1,start:2,end:9,size:3});
        tests.insert(9, Ring{num:1,start:2,end:9,size:3});
        tests.insert(10, Ring{num:2,start:10,end:25,size:5});
        tests.insert(25, Ring{num:2,start:10,end:25,size:5});
        tests.insert(26, Ring{num:3,start:26,end:49,size:7});
        
        for (arg, want) in tests {
            assert_eq!( Ring::for_square(arg), want );
        }
    }
    
    #[test]
    fn test_is_in_ring() {
        let mut ring = Ring::zero();
        assert!( ring.is_in_ring(1) );
        assert!( !ring.is_in_ring(2) );
        
        ring.next_ring();
        assert!( !ring.is_in_ring(1) );
        assert!( ring.is_in_ring(2) );
        assert!( ring.is_in_ring(9) );
        assert!( !ring.is_in_ring(10) );
    }
    
    #[test]
    fn test_distance_from_middle_of_side_not_in_ring() {
        let ring = Ring::zero();
        assert_eq!( ring.distance_from_middle_of_side(2), Err(RingError::NotInRing) );
    }
    
    #[test]
    fn test_distance_from_middle_of_side() {
        let mut ring = Ring::zero();
        ring.next_ring();
        ring.next_ring();
        
        let mut tests = HashMap::new();
        tests.insert( 11, 0 );
        tests.insert( 10, 1 );
        tests.insert( 12, 1 );
        tests.insert( 13, 2 );
        tests.insert( 14, 1 );
        tests.insert( 15, 0 );
        tests.insert( 16, 1 );
        tests.insert( 17, 2 );
        
        for (arg, want) in tests {
            assert_eq!( ring.distance_from_middle_of_side(arg).unwrap(), want );
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ring {
    num: u16,
    start: u32,
    end: u32,
    size: u16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RingError {
    NotInRing,
}

impl Error for RingError {
    fn description(&self) -> &str {
        match *self {
            RingError::NotInRing    => "square not in this ring",
        }
    }
}

impl fmt::Display for RingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RingError::NotInRing    => write!(f, "square not in this ring"),
        }
    }
}

impl Ring {
    fn zero() -> Ring {
        return Ring{ num: 0, start: 1, end: 1, size: 1 };
    }
    
    fn next_ring( &mut self ) {
        self.num += 1;
        self.start = self.end + 1;
        self.end += 8 * u32::from(self.num);
        self.size += 2;
    }
    
    pub fn for_square( square : u32 ) -> Ring {
        let mut ring = Ring::zero();

        while ring.end < square {
            ring.next_ring();
        }
    
        return ring;
    }
    
    pub fn is_in_ring( &self, square : u32 ) -> bool {
        return self.start <= square && square <= self.end;
    }
    
    pub fn distance_from_middle_of_side( &self, square : u32 ) -> Result<u32, RingError> {
        if !self.is_in_ring(square) {
            return Err(RingError::NotInRing);
        }
        if square == 1 {
            return Ok(0);
        }
        
        let middle : u32 = (u32::from(self.size)-1)/2;
        let mut pos = square - self.start + 1;
        pos %= u32::from(self.size) - 1;

        if pos > middle {
            return Ok(pos - middle);
        }
        else {
            return Ok(middle - pos);
        }
    }
    
    pub fn manhattan_distance( square : u32 ) -> u32 {
        let ring = Ring::for_square( square );
        return ring.distance_from_middle_of_side( square ).unwrap()
                + u32::from(ring.num);
    }
}
