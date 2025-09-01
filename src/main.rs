/*
 * A CLI Application written in rust that replicates the popular
 * "grep" CLI Application/tool in unix systems
 */
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let filename: &String = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);

    let contents: String = fs::read_to_string(path: filename)
        .expect(msg: "Something went wrong reading the file");
    println!("With text:\n()", contents);
}
