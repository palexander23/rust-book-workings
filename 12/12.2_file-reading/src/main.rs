// In this chapter we will be creating a version of the grep command line utility.
// The folder for each section will contain the same program in different states.
// The final section will contain the completed program.

// A fundamental part of grep is its ability to read arguments from the cli. 
// This section will focus on this functionality.
// There are arguments on cargo.io that could help with this but we shall 
// write our own for this tutorial.
// Arguments are retrieved using the standard lib function 
// std::env::args. This function returns an iterator containing all arguments.

use std::env;

// Another fundamental part of grep is reading files.
// This is completed using another part of the standard library called std::fs
// The function fs::read_to_string takes the name of a file and returns the 
// contents of the file as a string.

use std::fs;

fn main() {

    // Get string query and filename
    let arg_vec: Vec<String> = env::args().collect();
    
    let query = &arg_vec[1];
    let filepath = &arg_vec[2];

    println!("Checking for {} in {}", query, filepath);

    // Read in the contents of the file
    let contents = fs::read_to_string(filepath)
        .expect("Could not read file!");

    println!("File contents:\n{}", contents);
}
