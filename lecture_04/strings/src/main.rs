fn main() {
    let s1 = "Hello! How are you, 🦕?";  // type is immutable borrowed reference to a string slice: `&str`
    let s2 : &str = "Καλημέρα από την Βοστώνη και την DS210";  // here we make the type explicit
    
    println!("{}", s1);
    println!("{}\n", s2);

    // This doesn't work.  You can't do String = &str
    //let s3: String = "Does this work?";
    
    let s3: String = "Does this work?".to_string();
    println!("{}", s3);

    let s4: String = String::from("How about this?");
    println!("{}\n", s4);

    let s5: &str = &s3;
    println!("str reference to a String reference: {}\n", s5);
    
    // This won't work.  You can't index directly into a string slice. Why???
    // println!("{}", s1[3]);
    // println!("{}", s2[3]);

    // But you can index this way.
    println!("4th character of s1: {}", s1.chars().nth(3).unwrap());
    println!("4th character of s2: {}", s2.chars().nth(3).unwrap());
    println!("3rd character of s4: {}", s4.chars().nth(2).unwrap());
}
