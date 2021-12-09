use std::collections::HashMap;
mod compmap;
// the content of the module was here

use crate::compmap::*;

fn main() {
    let mut arbvec = Vec::new();
    
    let mut i = 0;
    let mut j = 0;
    //while i < 10 {
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
        // 0 = not in map
        // 1 - 6 = districts
        //i += 1;

    //}

    let mut arbitrary = CompMap {map: arbvec, numdists: 6, masks: HashMap::new()};
    arbitrary.generate_masks();
    let first_vec = arbitrary.masks.get(&2).unwrap();
    for i in 0..(first_vec.len()) {
        for j in 0..first_vec[i].len() {
            print!("{}", first_vec[i][j]);
        }
        println!();
    }
    println!("{}", arbitrary.compact(first_vec));
    match arbitrary.district_at(0, 0) {
        Ok(k) => println!("{}", k),
        Err(()) => println!("{}", "my bad bro")
    }
}
