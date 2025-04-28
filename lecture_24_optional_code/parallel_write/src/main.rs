use clap::Parser;
use rayon::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Matrix size
    #[arg(short, long, default_value_t = 64)]
    size: usize,

    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: usize,

    /// Display the matrix
    #[arg(short, long, default_value_t = false)]
    display: bool,
}

fn process(matrix: &mut [Vec<f64>], id: i32, _num_segs: i32) {
    for (_j, row) in matrix.iter_mut().enumerate() {
        for (_k, column) in row.iter_mut().enumerate() {
            *column = id as f64;
        }
    }
}

fn load_matrix(n: usize) -> Vec<Vec<f64>> {
    // Create an nXn array
    let arr = vec![vec![0.0f64; n]; n];
    return arr;
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

fn main() {
    let args = Args::parse();
    let size = args.size;
    let ncpus: i32 = args.threads as i32;
    let mut matrix: Vec<Vec<f64>> = load_matrix(size);
    if args.display {
        print_matrix(&matrix, size);
    }
    println!("Using {} cpus rows {} on this machine", ncpus, matrix.len());
    let chunk_size = matrix.len() / ncpus as usize;
    let mut veciter = matrix.chunks_mut(chunk_size);

    let now = Instant::now();
    rayon::scope(|s: &Scope| {
        for i in 0..ncpus {
            let slice = veciter.next().unwrap();
            s.spawn(move |_s: &Scope| process(slice, i, ncpus));
        }
    });
    let elapsed = now.elapsed();
    if args.display {
        print_matrix(&matrix, size);
    }
    println!("Elapsed: {:.2?}", elapsed);
}
