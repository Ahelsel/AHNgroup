use std::collections::HashMap;
pub struct CompMap {
    pub map: Vec<Vec<i32>>,
    pub numdists: i32,
    pub masks: HashMap<i32, Vec<Vec<i32>>>
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
        if (row == 0 || row == self.map.len() - 1) { return true; }
        if (col == 0 || col == self.map[row].len() - 1) { return true; }
        match wrapped_value {
            Ok(v) => {
                if (row == 0 || row == self.map.len() - 1) { return true; }
                if (col == 0 || col == self.map[row].len() - 1) { return true; }
                if (self.map[row - 1][col - 1] != self.map[row][col]) { return true; }
                if (self.map[row][col - 1] != self.map[row][col]) { return true; }
                if (self.map[row + 1][col - 1] != self.map[row][col]) { return true; }
                if (self.map[row - 1][col] != self.map[row][col]) { return true; }
                if (self.map[row + 1][col] != self.map[row][col]) { return true; }
                if (self.map[row - 1][col + 1] != self.map[row][col]) { return true; }
                if (self.map[row][col + 1] != self.map[row][col]) { return true; }
                if (self.map[row + 1][col + 1] != self.map[row][col]) { return true; }
                return false;
            },
            Err(()) => {
                panic!();
            },
        }
    }

    //this populates the HashMap masks by mapping each district number to a masked 2D vector map
    //a masked map fills all other values in the map as 0 except for the function 
    pub fn generate_masks(&mut self) {
        for i in 1..self.numdists + 1 {
            let mut mask = Vec::new();
            for j in 0..self.map.len() {
                let mut col = Vec::new();
                for k in 0..self.map[j].len() {
                    if self.map[j][k] == i {
                        col.push(1);
                    } else {
                        col.push(0);
                    }
                }
                mask.push(col);
            }
            self.masks.insert(i, mask);
        }
    }

    pub fn compact(&self, masked_map: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.is_border(i, j) {
                    perimeter += 1;
                }
            }
        } 
        return perimeter;
    }
}