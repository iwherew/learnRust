enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

enum IpAddrKind2{
    V4(u8,u8,u8,u8),
    V6(String)
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){}
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(four);

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let home2 = IpAddrKind2::V4(127,0,0,1);
    let loopback2 = IpAddrKind2::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move{x:12,y:24};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0,255,255);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // Cannot add `Option<i8>` to `i8`

    let c = Coin::Quarter(UsState::Alaska);
    println!("{}",value_in_cents(&c));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others"),
    }
    // 等价，只针对一种匹配模式
    if let Some(3) = v {
        println!("three");
    }
    // else{
    //     println!("others")
    // }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
fn value_in_cents(coin: &Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
        // _ 通配符，放最后匹配其他所有情况
        _ => 100,
    }
}

fn route(ip_kind: IpAddrKind){

}