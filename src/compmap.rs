pub struct CompMap {
    pub map: Vec<Vec<i32>>,
    pub numdists: i32,
} 

impl CompMap  {
    pub fn district_at(&self, row: usize, col: usize) -> i32 {
        return self.map[row][col];
    }
}