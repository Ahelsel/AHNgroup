mod compmap;
// the content of the module was here

use crate::compmap::*;

fn main() {
    let mut arbvec = Vec::new();
    
    let mut i = 0;
    let mut j = 0;
    while i < 10 {
        arbvec.push(vec![0,0,0,0,0,0,0,0,0]);
        arbvec.push(vec![1,0,0,0,2,2,5,5,5]);
        arbvec.push(vec![1,1,1,1,2,2,5,5,5]);
        arbvec.push(vec![1,1,1,1,1,2,2,6,6]);
        arbvec.push(vec![0,1,1,2,2,2,6,6,6]);
        arbvec.push(vec![0,0,3,3,3,6,6,6,0]);
        arbvec.push(vec![0,0,3,3,3,3,3,0,0]);
        arbvec.push(vec![0,4,4,4,4,3,0,0,0]);
        arbvec.push(vec![0,0,0,4,4,4,0,0,0]);
        arbvec.push(vec![0,0,0,0,4,4,0,0,0]);
        // 0 = not in map
        // 1 - 6 = districts
        i += 1;

    }

    let arbitrary = CompMap {map: arbvec, numdists: 1};
    match arbitrary.district_at(0, 0) {
        Ok(k) => println!("{}", k),
        Err(()) => println!("{}", "my bad bro")
    }
}

/* 
    // Display/Print function; 
    // Takes in the 2d Vector of Ints
    // Converts each row into a string.
    // A "0" will be represented by whitespace
    // Otherwise, print each line to the file, adding
    // a line break after each line.
    // Prints to a file called "printoutput.txt"
    // Takes in the 2d vec of ints
    fn printer(input: Vec<Vec<f64>>) -> std::io::Result<()> {
      let mut file = File::create("printoutput.txt");
      clean_file(file);
      let mut data: String = "";
      for row in input {
        let mut row_data = row_to_string(row);
        row_data.push('\n');
        data.push_str(row_data);
      }
      file.write_all(data.as_bytes())?;
      file.sync_all()?;
      Ok(());
    }
    
    // Takes in vector of ints, converts it to string
    // Make sure 0 is whitespace. 
    // If Every Character in the vector is 0, then return nothing
    // to avoid blank lines
    fn row_to_string() -> std::string {
      let mut toret = "";
      for ( int i = 0; i < input.size(); i++ ) {
        if (input.at(i) == '0') {
          toret.append(" ");
        } else {
          toret.append(input.at(i))
        }
      }
      return toret;
    }

    
    // MAYBE
    // Takes in the original data, and prints only the areas that changed before && after the algorithm is run, 
    // leaving out the unaffected areas.
    // MAYBE
    fn print_differences() -> std::io::Result<()> {
      todo!();
    }
    
    //Clears contents of the file so we can print something else on it
    fn clean_file(input: File) -> std::io::Result<()> {
      input.truncate();
    }

    
    // Prints out a side by side representation
    // of the data before and after the algorithm
    // is called on it for restructuring  
    fn print_sxs() -> std::io::Result<()> {
      todo!();
    }
*/
