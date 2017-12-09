#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral() {
        let wants = vec![
            (0,0),
            (1,0), (1,1),
            (0,1), (-1,1),
            (-1,0), (-1,-1),
            (0,-1), (1,-1),
            (2,-1), (2,0), (2,1), (2,2),
            (1,2), (0,2), (-1,2), (-2,2),
            (-2,1), (-2,0), (-2,-1), (-2,-2),
            (-1,-2), (0,-2), (1,-2), (2,-2),
            (3,-2),
        ].into_iter().map( |a| Point{ x: a.0, y: a.1 } );
        
        let mut spiral = Spiral::new();
        
        for want in wants {
            let have = spiral.next().unwrap();
            println!("want: {:?}, have: {:?}", want, have);
            assert_eq!( have, want );
        }
    }
    
    #[test]
    fn test_point_neighbors() {
        let mut want : Vec<Point> = vec![
            (0, 2),  (1,2),  (2,2),
            (0, 1),          (2,1),
            (0, 0),  (1,0),  (2,0),
        ].into_iter()
         .map( |a| Point{ x: a.0, y: a.1 } )
         .collect();
        
        let point = Point{ x: 1, y: 1 };
        let mut neighbors : Vec<Point> = point.neighbors().collect();
        
        assert_eq!( neighbors.sort(), want.sort() );
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn neighbors(&self) -> PointNeighbors {
        const NEIGHBOR_MASK : [(i32,i32); 8] = [
            (-1,1),  (0,1),  (1,1),
            (-1,0),          (1,0),
            (-1,-1), (0,-1), (1,-1),
        ];
        
        return PointNeighbors{
            point: self.clone(),
            mask: NEIGHBOR_MASK,
            pos: 0
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PointNeighbors {
    point: Point,
    mask: [(i32,i32); 8],
    pos: usize,
}

impl Iterator for PointNeighbors {
    type Item = Point;
    
    fn next( &mut self ) -> Option<Point> {
        if self.pos >= self.mask.len() {
            return None;
        }
        
        let mask = self.mask[self.pos];
        let neighbor = Point{
            x: self.point.x + mask.0,
            y: self.point.y + mask.1,
        };
        
        self.pos += 1;
        
        return Some(neighbor);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Spiral {
    pos: Point,
    ring: i32,
}

impl Spiral {
    pub fn new() -> Spiral {
        // This just happens to make the first
        // next() be 0,0. If the next() logic
        // changes, this initial condition should
        // also change.
        Spiral { ring: 0, pos: Point{x: -1, y: 0} }
    }
}

impl Iterator for Spiral {
    type Item = Point;
    
    fn next(&mut self) -> Option<Point> {
        // Next ring
        if self.pos.x == self.ring && self.pos.y == -self.ring {
            self.ring += 1;
            self.pos.x += 1;
        }
        // Bottom
        else if self.pos.y == -self.ring {
            self.pos.x += 1;
        }
        // Left
        else if self.pos.x == -self.ring {
            self.pos.y -= 1;
        }
        // Top
        else if self.pos.y == self.ring {
            self.pos.x -= 1;
        }
        // Right
        else if self.pos.x == self.ring {
            self.pos.y += 1;
        }
        else {
            panic!(format!("Spiral should not be here: {:?}", self));
        }
        
        return Some(self.pos.clone())
    }
}
