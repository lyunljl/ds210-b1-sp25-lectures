use clap::Parser;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Matrix size
    #[arg(short, long, default_value_t = 64)]
    size: usize,

    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: i32,

    /// Method of parallelism (x=exclusive, r=reader, w=writer)
    #[arg(short, long, default_value_t = 'x')]
    method: char,

    /// Display the matrix
    #[arg(short, long, default_value_t = false)]
    display: bool,
}

fn process(matrix: &Vec<Vec<f64>>, id: i32, num_segs: i32) {
    let seg = matrix.len() / num_segs as usize;
    let my_start = seg * id as usize;
    let my_end = my_start + seg;
    println!(
        "Thread {} total rows {} running {} on rows {} {}",
        id,
        matrix.len(),
        num_segs,
        my_start,
        my_end
    );

    let mut myval = 0.0;
    for j in my_start..my_end {
        for k in 0..matrix[j].len() {
            myval += matrix[j][k];
        }
    }
    println!("The sum from thread {} is {}", id, myval);
}

fn print_matrix(matrix: &Vec<Vec<f64>>, n: usize) {
    let mut output = String::with_capacity(16 * n * (n + 1));
    for i in 0..n {
        for j in 0..n {
            let s = format!("{: >9.2}", matrix[i][j]);
            output.push_str(&s);
            output.push(' ');
        }
        output.push_str("\n");
    }
    output.push_str("\n\n\n");
    print!("{}", output);
}

fn load_matrix(n: usize) -> Vec<Vec<f64>> {
    // Create an nXn array
    let arr = vec![vec![1.0f64; n]; n];
    return arr;
}

fn main() {
    let args = Args::parse();
    let ncpus: i32 = args.threads;
    let method: char = args.method;
    let n: usize = args.size;
    // Dimensions of the matrix
    let matrix1 = Arc::new(Mutex::new(load_matrix(n)));
    let matrix2 = Arc::new(RwLock::new(load_matrix(n)));
    println!(
        "Using {} cpus rows {} method {} on this machine",
        ncpus,
        matrix1.lock().unwrap().len(),
        method
    );
    if args.display {
        print_matrix(&matrix1.lock().unwrap(), n);
    }
    let mut handles = vec![];

    let now = Instant::now();
    if method == 'x' {
        for i in 0..ncpus {
            let matrix1 = matrix1.clone();
            let handle = thread::spawn(move || {
                let matrix1 = matrix1.lock().unwrap();
                process(&matrix1, i, ncpus);
            });
            handles.push(handle);
        }
    } else {
        for i in 0..ncpus {
            let matrix2 = matrix2.clone();
            let handle = thread::spawn(move || {
                if method == 'r' {
                    let matrixp = matrix2.read().unwrap();
                    process(&matrixp, i, ncpus);
                } else {
                    let matrixp = matrix2.write().unwrap();
                    process(&matrixp, i, ncpus);
                }
            });
            handles.push(handle);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = now.elapsed();
    if args.display {
        print_matrix(&matrix1.lock().unwrap(), n);
    }
    println!("Elapsed: {:.2?}", elapsed);
}
