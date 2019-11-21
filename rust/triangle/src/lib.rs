use std::collections::HashSet;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        if sides.iter().all(|&s| s != 0) && sides[0] + sides[1] >= sides[2] {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.different_edges_count() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.different_edges_count() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.different_edges_count() == 3
    }

    fn different_edges_count(&self) -> usize {
        self.sides.iter().cloned().collect::<HashSet<u64>>().len()
    }
}
