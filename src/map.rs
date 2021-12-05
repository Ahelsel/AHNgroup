pub struct Map {
    map: Vec<Vec<i32>>
}

impl Map {
    pub fn set_data(&mut self) {
        self.map = vec![vec![0,0,0,0,1,1,1,2,2,2,2,2,3,3,4,4,4,4,4,5],
                        vec![0,0,1,1,1,1,1,1,1,2,2,2,3,3,3,4,4,4,5,5],
                        vec![0,0,1,1,1,1,1,1,1,2,2,2,3,3,3,4,4,4,5,5]];
    }
    //functions for algorithm go here...
}