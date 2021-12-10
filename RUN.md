1. git clone
2. cargo run




PRINT STATEMENT HOW TO: 

  To print the output of a dataset, call it like: 
        ComputationalRedistricting::printer(&arbvec);
        
  After doing this, the output will be in printout.txt file that has been generated. To save a file, call the save function like this:
        ComputationalRedistricting::save_file("DesiredName"); 
  
  This will save whatever is currently in "printoutput.txt" to a new file called "DesiredName.txt"
  You should not add .txt to the name, this will be added by the function automatically. 
  If you call the save function twice with the same name, the data in that save file will be overwritten with 
  whatever is currently held in "printoutput.txt"
      
