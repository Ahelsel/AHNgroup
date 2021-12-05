mod compmap;
// the content of the module was here

use crate::compmap::*;

fn main() {
    let mut arbvec = Vec::new();
    
    let mut i = 0;
    let mut j = 0;
    while i < 10 {
        arbvec.push(Vec::new());
        while j < 10 {    
            arbvec[i].push(1);
            j += 1;
        }
        i += 1;
    }
    let arbitrary = CompMap {map: arbvec, numdists: 1};
    println!("{}", arbitrary.district_at(0, 0));
}
