    mod compmap;
    
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    
    /*
    Display/Print function; 
    Takes in the 2d Vector of Ints
    Converts each row into a string.
    A "0" will be represented by whitespace
    Otherwise, print each line to the file, adding
    a line break after each line.

    Prints to a file called "printoutput.txt"
    Everytime this function is called, the printoutput.txt file is
    overwritten. Call save_file("filename_to_save_to") if you want to save it's contents.
    */
    pub fn printer(input: Vec<Vec<i32>>) -> std::io::Result<()> {
      if Path::new("printoutput.txt").exists() {
        println!("Overwriting printoutput.txt with new data.");
      } else {
        println!("Printing data to printoutput.txt.");
      }

      let file = File::create("printoutput.txt");
      let mut file = match file {
        Ok(file) => file,
        Err(file) => {
          panic!();
        }
      };
      file.set_len(0);
      let mut data: String = "".to_string();
      for row in input {
        let mut row_data = row_to_string(row);
        if !row_data.trim().is_empty() {
          row_data.push('\n');
          data.push_str(&row_data);
        }
      }
      file.write_all(data.as_bytes())?;
      file.sync_all()?;
      println!("Successfully printed data.");
      Ok(())
    }

    /*
    Takes in vector of ints, converts it to string
    Make sure 0 is whitespace. 
    If Every Character in the vector is 0, then return nothing
    to avoid blank lines
    TODO: MAKE SURE ALL DATA IS DIGITS
    */
    fn row_to_string(input: Vec<i32>) -> String {
      let mut toret: String = "".to_string();
      for i in input {
        if i == 0 {
          toret.push_str(" ");
        } else {
          toret.push_str(&i.to_string());
        }
      }
      return toret;
    }

    
    /*
    Saves results of current "printoutput.txt" to a file
    with name of user's choosing. 
    For example, calling 'save_file("test")' will create a file
    called "test.txt" and save the current contents of printoutput.txt to it.
    If you call save_file() using the name of an already existing .txt file, the existing
    file is overwritten.
    */
    pub fn save_file(name: &str) -> std::io::Result<()> {
      let filename: String = (name.to_owned() + ".txt").to_string();
      let file = File::create(&filename);
      let mut file = match file {
        Ok(file) => file,
        Err(file) => {
          panic!();
        }
      };
      fs::copy("printoutput.txt", filename);
      Ok(())
    }
    
