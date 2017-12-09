#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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
        ];
        
        let mut spiral = Spiral::new();
        
        for want in wants {
            let have = spiral.next().unwrap();
            println!("want: {:?}, have: {:?}", want, have);
            assert_eq!( have, want );
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Spiral {
    x: i32,
    y: i32,
    ring: i32,
}

impl Spiral {
    pub fn new() -> Spiral {
        // This just happens to make the first
        // next() be 0,0. If the next() logic
        // changes, this initial condition should
        // also change.
        Spiral { ring: 0, x: -1, y: 0 }
    }
}

impl Iterator for Spiral {
    type Item = (i32,i32);
    
    fn next(&mut self) -> Option<(i32,i32)> {
        // Next ring
        if self.x == self.ring && self.y == -self.ring {
            self.ring += 1;
            self.x += 1;
        }
        // Bottom
        else if self.y == -self.ring {
            self.x += 1;
        }
        // Left
        else if self.x == -self.ring {
            self.y -= 1;
        }
        // Top
        else if self.y == self.ring {
            self.x -= 1;
        }
        // Right
        else if self.x == self.ring {
            self.y += 1;
        }
        else {
            panic!(format!("Spiral should not be here: {:?}", self));
        }
        
        return Some((self.x, self.y))
    }
}
