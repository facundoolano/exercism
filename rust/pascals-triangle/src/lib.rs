pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::new();
        let row_count = row_count as usize;

        for row in 0..row_count {
            rows.push(Vec::new());

            for column in 0..=row {
                let value = match (row, column) {
                    (_, 0) => 1,
                    (_, column) if column == row => 1,
                    (_row, _column) => rows[row - 1][column - 1] + rows[row - 1][column],
                };

                rows[row].push(value);
            }
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
