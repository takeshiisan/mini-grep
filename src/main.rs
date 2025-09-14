/*
 * A CLI Application written in rust that replicates the popular
 * "grep" CLI Application/tool in unix systems
 */
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
