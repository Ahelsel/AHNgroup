use std::collections::HashMap;
mod compmap;
// the content of the module was here

use crate::compmap::*;

fn main() {
    let mut arbvec = Vec::new();
    let mut arbpop = Vec::new();

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

    arbpop.push(vec![0,0,0,0,0,0,0,0,0,0]);
    arbpop.push(vec![1,0,0,0,1,1,1,3,2,1]);
    arbpop.push(vec![1,1,1,1,2,1,1,2,2,1]);
    arbpop.push(vec![1,1,1,1,1,2,1,1,1,1]);
    arbpop.push(vec![0,1,1,1,2,3,2,2,1,0]);
    arbpop.push(vec![0,0,2,1,2,1,3,2,0,0]);
    arbpop.push(vec![0,0,3,4,3,2,2,0,0,0]);
    arbpop.push(vec![0,4,2,3,2,3,0,0,0,0]);
    arbpop.push(vec![0,0,0,2,1,4,0,0,0,0]);
    arbpop.push(vec![0,0,0,0,4,2,0,0,0,0]);

    // let mut arbitrary = CompMap {map: arbvec, population: arbpop, numdists: 6, masks: HashMap::new()};
    // arbitrary.generate_masks();
    // let first_vec = arbitrary.mask_combiner(1, 3);
    // for i in 0..(first_vec.len()) {
    //     for j in 0..first_vec[i].len() {
    //         print!("{}", first_vec[i][j]);
    //     }
    //     println!();
    // }

    ComputationalRedistricting::printer(&arbvec);
    ComputationalRedistricting::save_file("test1");
}

