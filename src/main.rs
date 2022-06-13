use std::io;
use std::io::prelude::*;



fn main() {
    
    print!("Input string: ");
    
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut string: String = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    println!("{}", string.trim());
    let s = string.trim();
    

    println!("{}","This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal."
        .matches(&s).count());
    
}
