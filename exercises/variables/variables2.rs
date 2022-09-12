// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

use std::cmp::Ordering;

fn main() {
    let x = 10;
    match x.cmp(&10) {
        Ordering::Less => println!("x is not ten!"),
        Ordering::Equal => println!("x is ten!"),
        Ordering::Greater => println!("x is not ten!"),
    }
    
}
