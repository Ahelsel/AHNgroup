use std::collections::HashMap;
mod compmap;
// the content of the module was here

use crate::compmap::*;

fn main() {
    let mut arbvec = Vec::new();
    
    let mut i = 0;
    let mut j = 0;

    // 0 = not in map
    // 1 - 6 = districts

    arbvec.push(vec![0,0,0,0,0,0,0,0,0,0]);
    arbvec.push(vec![1,0,0,0,2,2,5,5,5,5]);
    arbvec.push(vec![1,1,1,1,2,2,5,5,5,6]);
    arbvec.push(vec![1,1,1,1,1,2,2,6,6,6]);
    arbvec.push(vec![0,1,1,2,2,2,6,6,6,0]);
    arbvec.push(vec![0,0,3,3,3,6,6,6,0,0]);
    arbvec.push(vec![0,0,3,3,3,3,3,0,0,0]);
    arbvec.push(vec![0,4,4,4,4,3,0,0,0,0]);
    arbvec.push(vec![0,0,0,4,4,4,0,0,0,0]);
    arbvec.push(vec![0,0,0,0,4,4,0,0,0,0]);

    let mut arbitrary = CompMap {map: arbvec, numdists: 6, masks: HashMap::new()};
    arbitrary.generate_masks();
    let first_vec = arbitrary.mask_combiner(1, 3);
    for i in 0..(first_vec.len()) {
        for j in 0..first_vec[i].len() {
            print!("{}", first_vec[i][j]);
        }
        println!();
    }
}
