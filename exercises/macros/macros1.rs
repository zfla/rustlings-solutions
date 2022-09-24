// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.



// macros are codes that write other code, known as "metaprogramming"

#[macro_export] // must be used to bring the macro into scope
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
