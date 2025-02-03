fn main() {
    let x = 9;
    let y = 16;
    
    print!("Hello, DS210!\n");       // Need to include the newline character
    println!("Hello, DS210!\n");     // The newline character here is redundant

    println!("{} plus {} is {}", x, y, x+y);  // print with formatting placeholders
    //println!("{x} plus {y} is {x+y}");      // error: cannot use `x+y` in a format string
    println!("{x} plus {y} is {}\n", x+y);      // but you can put variable names in the format string
    
    println!("{:?} plus {:?} is {:?}\n", x, y, x+y);  // {:?} format specifier for debug

    println!("Hexadecimal: 0x{:X} plus 0x{:X} is 0x{:X}", x, y, x+y);  // {:X} format specifier for uppercase hexadecimal
    println!("Octal: 0o{:o} plus 0o{:o} is 0o{:o}", x, y, x+y);  // {:o} format specifier for octal
    println!("Binary: {:b} plus {:b} is {:b}\n", x, y, x+y);  // {:b} format specifier for binary
    
    println!("pointer to x: {:p}", &x);   // {:p} format specifier for pointer
    println!("pointer to y: {:p}", &y);
    println!("pointer to x+y: {:p}\n", &(x+y));
    
    let z = format!("{} plus {} is {}\n", x, y, x+y);  // format! is a macro that returns a string
    println!("{}", z);  
    
    eprint!("E {} plus {} is {}\n", x, y, x+y);      // eprint! is a macro that prints to the standard error stream
    eprintln!("E {} plus {} is {}\n", x, y, x+y);    // eprintln! is a macro that prints to the standard error stream and adds a newline character
}
