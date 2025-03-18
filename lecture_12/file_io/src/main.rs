use std::fs::File;        // object providing access to an open file on filesystem
use std::io::prelude::*;  // imports common I/O traits
use rand::Rng;            // trait for random number generation. defines core RNG interface

fn generate_file(path: &str, n: usize) {
    // Generate a random file of edges for vertices 0.n

    // Create a file at `path` and return Result<File> or error
    // .expect() unwraps the Result<File> or prints error and panics
    let mut file = File::create(path).expect("Unable to create file");
    
    for i in 0..n {
        // Randomly pick how many neighbors this node have up to 19
        let rng = rand::thread_rng().gen_range(0..20) as usize;
        
        for _ in 0..rng {
            // Randomly select a neighbor (even with duplicates but not to ourselves)
            let neighbor = rand::thread_rng().gen_range(0..n) as usize;
            if neighbor != i {
                let s: String = format!("{} {}\n", i, neighbor);
                file.write_all(s.as_bytes()).expect("Unable to write file");
            }
        }
    }
}
    
fn read_file(path: &str) -> Vec<(u32, u32)> {
    // Read the file and return a vector of tuples (u32, u32)

    let mut result: Vec<(u32, u32)> = Vec::new();

    let file = File::open(path).expect("Could not open file");

    // Create a buffered reader for the file
    // .lines() returns an iterator over the lines of the file
    let buf_reader = std::io::BufReader::new(file).lines();

    for line in buf_reader { // returns a Result<String, Error>

        // Unwrap or print error
        let line_str = line.expect("Error reading");

        // Split the line into a vector of strings
        let v: Vec<&str> = line_str
            .trim() // remove whitespace
            .split(' ') // split on space
            .collect(); // collect into a vector (convert iterator to vector)

        // Parse the first string as a u32
        let x = v[0].parse::<u32>().unwrap();

        // Parse the second string as a u32
        let y = v[1].parse::<u32>().unwrap();

        // Add the tuple to the result vector
        result.push((x, y));
    }
    return result;
}
    
fn main() {
    println!("Generating file");
    generate_file("list_of_edges.txt", 10);

    println!("Reading file");
    let edges = read_file("list_of_edges.txt");
    println!("{:?}", edges);
}
