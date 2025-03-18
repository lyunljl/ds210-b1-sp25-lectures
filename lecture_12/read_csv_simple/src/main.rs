// This is a modified version of the example from the docs of crate `csv`.
// At the time of writing (2022-04-06), this was the second example at
// https://crates.io/crates/csv . The only thing that was modified was
// struct Record below. This was a demo for DS-210 at Boston University.

use std::env;
use std::error::Error;
use std::process;

/*
 * The '?' after .from_path(path) is a shortcut for propagating errors.
 * See https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
 * in §9.2 on Recoveragle Errors with Result, https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
 */

fn example(path: &str, hh: bool) -> Result<(), Box<dyn Error>> {
    println!("Using file {}", path);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(hh)
        .flexible(true) // allow for variable numbers of columns
        .from_path(path)?;
    for result in rdr.records() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let r = result.unwrap();
        println!("{:?}", r);
        for element in &r {
	        println!("{:?}", element);
        }
    }
    Ok(())
}

/*
 * To use: cargo run -- <file.csv> <has_headers: boolean>
 * 
 * Example:
 * cargo run -- pizza.csv true
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[1];
    let has_headers: &str = &args[2];
    let mut hh = false;
    if has_headers == "true" {
       hh = true;
    }
    if let Err(err) = example(path, hh) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
