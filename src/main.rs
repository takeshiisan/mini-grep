/*
 * A CLI Application written in rust that replicates the popular
 * "grep" CLI Application/tool in unix systems
 */
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
}
