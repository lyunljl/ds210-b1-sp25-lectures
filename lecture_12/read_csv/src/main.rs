// This is a modified version of the example from the docs of crate `csv`.
// At the time of writing (2022-04-06), this was the second example at
// https://crates.io/crates/csv . The only thing that was modified was
// struct Record below. This was a demo for DS-210 at Boston University.

use std::env;
use std::error::Error;
use std::process;

use serde::Deserialize;

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
struct Record {
    Name: String,
    Number: Option<u8>,
    PPG: Option<f64>,
    YearBorn: Option<u16>,
    TotalPoints: Option<u32>,
    LikesPizza: Option<u8>,
}

fn example(path: &str, hh: bool) -> Result<(), Box<dyn Error>> {
    println!("Using file {}", path);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(hh)
        .flexible(true)
        .from_path(path)?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

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
