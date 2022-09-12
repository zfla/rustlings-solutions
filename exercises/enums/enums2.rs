// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u8, y: u8 }, // struct
    Echo(String), // string
    ChangeColor(u8, u8, u8), // tuple
    Quit, // nothing
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
