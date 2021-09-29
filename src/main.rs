#[derive(Debug)]
enum IpAddr {
    v4(String),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    // impl works for both enum and struct and is used to create a function related to the struct/enum type.
    fn func() {
        println!("Hello kids!!");
    }
}

fn main() {
    let four = IpAddr::v4(String::from("localhost"));
    println!("{:#?}", four);
}
