use chrono::Local;
use clap::Parser;
use rayon::*;
use std::ops::Sub;
use std::thread;
use std::time;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Board size
    #[arg(short, long, default_value_t = 64)]
    size: usize,

    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: usize,

    /// Display the board
    #[arg(short, long, default_value_t = false)]
    display: bool,

    /// Lag between iterations in milliseconds
    #[arg(short, long, default_value_t = 0)]
    lag: u64,

    // Number of iterations
    #[arg(short, long, default_value_t = 10)]
    iters: u32,
}

pub struct Board {
    pub n: usize,
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(n: usize) -> Board {
        let board = vec![vec![0; n]; n];
        Board { n, board }
    }

    pub fn parallel_update(&self, start: usize, end: usize, new_board_chunk: &mut [Vec<i32>]) {
        for i in start..end {
            for j in 0..self.n {
                let im = (i + self.n - 1) % self.n;
                let ip = (i + 1) % self.n;
                let jm = (j + self.n - 1) % self.n;
                let jp = (j + 1) % self.n;
                let sum = self.board[im][jm]
                    + self.board[im][j]
                    + self.board[im][jp]
                    + self.board[i][jm]
                    + self.board[i][jp]
                    + self.board[ip][jm]
                    + self.board[ip][j]
                    + self.board[ip][jp];

                // this is the part where I may have gotten wrong
                if sum == 2 {
                    new_board_chunk[i - start][j] = self.board[i][j];
                } else if sum == 3 {
                    new_board_chunk[i - start][j] = 1;
                } else {
                    new_board_chunk[i - start][j] = 0;
                }
            }
        }
    }

    pub fn update(&self, t_num: usize) -> Board {
        let mut result = Board::new(self.n);

        let mut block_size = self.n / t_num;
	if self.n % t_num != 0 {
	   block_size += 1;
        }
        let mut start = 0;

        rayon::scope(|s| {
            // we need to use `chunks_mut` to split the board into parts
            // all with the same lifetime as the `result`
            for (t, new_board_chunk) in result.board.chunks_mut(block_size).enumerate() {
                let mut end = start + block_size;

                if t == t_num - 1 {
                    end = self.n;
                }
		
                s.spawn(move |_s| {
                    self.parallel_update(start, end, new_board_chunk);
                });

                start = start + block_size;
            }

            // all threads are automatically joined here
        });

        result
    }

    pub fn initialize_glider(&mut self) {
        self.board[0][1] = 1;
        self.board[1][2] = 1;
        self.board[2][0] = 1;
        self.board[2][1] = 1;
        self.board[2][2] = 1;
    }
}

fn main() {
    let dt1 = Local::now();

    let args = Args::parse();
    let mut board = Board::new(args.size);
    let mut iters: u32 = 0;

    board.initialize_glider();
    loop {
        if args.display == true {
            board_render(&board, iters);
        }

        board = board.update(args.threads);
	if args.lag != 0 {
          thread::sleep(time::Duration::from_millis(args.lag));
        }
        iters += 1;
        if iters >= args.iters {
            break;
        }
    }
    if args.display == true {
        board_render(&board, iters);
    }
    let dt2 = Local::now();
    println!(
        "It took {:?} milliseconds to run",
        (dt2.sub(dt1)).num_milliseconds()
    );
}

fn board_render(board: &Board, iters: u32) {
    print!("Iteration {}\n  ", iters);
    let mut output = String::with_capacity(board.n * (board.n + 1));
    for i in 0..board.n {
        for j in 0..board.n {
            let ch;
            if board.board[i][j] == 0 {
                ch = '_';
            } else {
                ch = 'x';
            }
            output.push(ch);
        }
        output.push_str("\n  ");
    }
    output.push_str("\n\n\n");
    print!("{}", output);
}
