use std::collections::HashMap;
use std::collections::HashSet;
pub struct CompMap {
    pub map: Vec<Vec<i32>>,
    pub population: Vec<Vec<i32>>,
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

    pub fn compact(&self, masked_map: &Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for i in 0..masked_map.len() {
            for j in 0..masked_map[i].len() {
                if masked_map[i][j] == 1 && self.is_border(i, j) {
                    perimeter += 1;
                }
            }
        } 
        return perimeter;
    }

    pub fn mask_combiner(&self, first: i32, second: i32) -> Vec<Vec<i32>> {
        let mut first_mask: Vec<Vec<i32>> = Vec::new();
        let mut second_mask: Vec<Vec<i32>> = Vec::new();
        match self.masks.get(&first) {
            Some(mask) => first_mask = mask.to_vec(),
            None => {println!("{}", "first not there!"); panic!();},
        }
        match self.masks.get(&second) {
            Some(mask) => second_mask = mask.to_vec(),
            None => {println!("{}", "second not there!"); panic!();},
        }
        let mut final_mask: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.map.len() {
            final_mask.push(Vec::new());
            for j in 0..self.map[i].len() {
                final_mask[i].push(first_mask[i][j] + second_mask[i][j]);
            }
        }
        return final_mask;
    }

    pub fn borders(&self, first: i32) -> HashSet<i32> {
        let mut borders : HashSet<i32> = HashSet::new();
        for row in 0..(self.map.len()) {
            for col in 0..self.map[row].len() {
                if self.map[row][col] == first {
                    if (self.map[row - 1][col - 1] != self.map[row][col]) { borders.insert(self.map[row - 1][col - 1]); }
                    if (self.map[row][col - 1] != self.map[row][col]) { borders.insert(self.map[row][col - 1]); }
                    if (self.map[row + 1][col - 1] != self.map[row][col]) { borders.insert(self.map[row + 1][col - 1]); }
                    if (self.map[row - 1][col] != self.map[row][col]) { borders.insert(self.map[row - 1][col]); }
                    if (self.map[row + 1][col] != self.map[row][col]) { borders.insert(self.map[row + 1][col]); }
                    if (self.map[row - 1][col + 1] != self.map[row][col]) { borders.insert(self.map[row - 1][col + 1]); }
                    if (self.map[row][col + 1] != self.map[row][col]) { borders.insert(self.map[row][col + 1]); }
                    if (self.map[row + 1][col + 1] != self.map[row][col]) { borders.insert(self.map[row + 1][col + 1]); }
                }
            }
        }
        return borders;
    }

    pub fn has_border(&self, first: i32, second: i32) -> bool {
        let borders : HashSet<i32> = self.borders(first);
        return borders.contains(&second);
    }

    pub fn redistrict(&self, first: i32, second: i32) {
        if !self.has_border(first, second) {return ();}
        else {
            let combined_mask : Vec<Vec<i32>> = self.mask_combiner(first, second);
            let mut mask_overload : Vec<Vec<i32>> = Vec::new();
            let mut mask_changer : Vec<Vec<i32>> = Vec::new();
            for row in 0..(combined_mask.len()) {
                mask_overload.push(Vec::new());
                mask_changer.push(Vec::new());
                for col in 0..combined_mask[row].len() {
                    mask_overload[row].push(combined_mask[row][col] * self.map[row][col]);
                    mask_changer[row].push(combined_mask[row][col] * self.map[row][col]);
                }
            }
        }
    }
}