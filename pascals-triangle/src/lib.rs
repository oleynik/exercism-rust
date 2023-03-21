pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut triangle = vec![];
        for i in 1..=row_count {
            if i == 1 {
                triangle.push(vec![1]);
                continue;
            }
            let mut row: Vec<u32> = vec![];
            for j in 0..i {
                if j == 0 {
                    row.push(1);
                    continue;
                }
                let prev = triangle.get(i - 2);
                let value =
                    prev.unwrap().get(j - 1).unwrap_or(&0) + prev.unwrap().get(j).unwrap_or(&0);
                row.push(value);
            }
            triangle.push(row);
        }
        Self(triangle)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
