pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for _ in 0..self.0 {
            if let Some(prev) = rows.last() {
                let mut next = (&prev[0..prev.len() - 1])
                    .iter()
                    .zip((&prev[1..]).iter())
                    .map(|(l, r)| l + r)
                    .collect::<Vec<u32>>();
                next.push(1);
                next.insert(0, 1);
                rows.push(next);
            } else {
                rows.push(vec![1]);
            }
        }
        rows
    }
}
