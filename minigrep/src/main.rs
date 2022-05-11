use std::env;

// create the main function
fn main(){
    // create args of type string and collect them in a vector
    let args: Vec<String> = env::args().collect();
    // store the first arguments in a variable called query
    // and the second argument in a variable called filename
    let query = &args[1];
    let filename = &args[2];

    // print the query and the filename
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
