// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// visit 
// https://veykril.github.io/tlborm/syntax-extensions.html
// for more details on macros

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // required to separate the different "arms"
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
