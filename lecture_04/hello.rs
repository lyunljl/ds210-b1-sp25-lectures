// And some input routines
// Unfortunately jupyter notebook does not have support for reading from the terminal with Rust at this point.
// So this is for demo purposes
use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    print!("What's your name? ");
    io::stdout().flush().expect("Error flushing");
    let _ =io::stdin().read_line(&mut user_input);
    println!("Hello, {}!", user_input.trim());
}
