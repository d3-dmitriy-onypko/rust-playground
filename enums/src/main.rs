#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
    
}

fn main() {
    let x = 8;
    let some_value:Option<i32> = Option::None;
    let multiply: i32 = x + some_value.unwrap();
    println!("{}", multiply);
    // let m = Message::Write(String::from("hello"));
    // m.call();
}
