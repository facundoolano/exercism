pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];

        for row in 0..row_count as usize {
            let columns = (0..=row)
                .map(|column| Self::value_at(&rows, row, column))
                .collect();
            rows.push(columns);
        }

        Self { rows }
    }

    fn value_at(rows: &[Vec<u32>], row: usize, column: usize) -> u32 {
        match (row, column) {
            (_, 0) => 1,
            (_, column) if column == row => 1,
            (row, column) => rows[row - 1][column - 1] + rows[row - 1][column],
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
