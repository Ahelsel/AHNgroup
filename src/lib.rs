    mod compmap;
    
    use std::fs::File;
    use std::io::Write;
    
    /*
    Display/Print function; 
    Takes in the 2d Vector of Ints
    Converts each row into a string.
    A "0" will be represented by whitespace
    Otherwise, print each line to the file, adding
    a line break after each line.

    Prints to a file called "printoutput.txt"
    */
    pub fn printer(input: Vec<Vec<i32>>) -> std::io::Result<()> {
      let file = File::create("printoutput.txt");
      // VVV Did this because before file was being a Result<()> instead of a File
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
    Save results to different file so its not erase
    (User has to call this command its not saved automatically);
    */
    /*
    static mut NUM_SAVED_FILES: i32 = 0;
    fn save_file() {
      unsafe {
        let mut filename = "save".push_str(&NUM_SAVED_FILES.to_string()).push_str(".txt");
      }
      fs::copy("printoutput.txt", /saves/filename);
      unsafe {
        NUM_SAVED_FILES += 1;
      }
    }
    */
