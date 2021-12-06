pub struct CompMap {
    pub map: Vec<Vec<i32>>,
    pub numdists: i32,
} 

impl CompMap  {
    pub fn district_at(&self, row: usize, col: usize) -> Result<i32, ()> {
        if (row < 0 || row > self.map.len() - 1) { return Err(()); }
        if (col < 0 || col > self.map[0].len() - 1) { return Err(()); }
        return Ok(self.map[row][col]);
    }
    fn is_border(&self, row: usize, col: usize) -> bool {
        let wrapped_value = self.district_at(row, col);
        let mut value = 0;
        if (row == 0 || row == self.map.len() - 1) { return false; }
        if (col == 0 || col == self.map[row].len() - 1) { return false; }
        match wrapped_value {
            Ok(v) => {
                if (row == 0 || row == self.map.len() - 1) { return true; }
                if (col == 0 || col == self.map[row].len() - 1) { return true; }
                if (self.map[row - 1][col - 1] != self.map[row][col]) { return false; }
                if (self.map[row][col - 1] != self.map[row][col]) { return false; }
                if (self.map[row + 1][col - 1] != self.map[row][col]) { return false; }
                if (self.map[row - 1][col] != self.map[row][col]) { return false; }
                if (self.map[row + 1][col] != self.map[row][col]) { return false; }
                if (self.map[row - 1][col + 1] != self.map[row][col]) { return false; }
                if (self.map[row][col + 1] != self.map[row][col]) { return false; }
                if (self.map[row + 1][col + 1] != self.map[row][col]) { return false; }
                return true;
            },
            Err(()) => {
                return false;
            },
        }
    }
}