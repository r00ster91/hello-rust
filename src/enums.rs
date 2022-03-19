// since we get a lot of warnings related to unused code,
// let's disable those warnings using these annotation:
#![allow(dead_code)]
#![allow(unused_variables)]
// ones that look like `#![` are crate-level allow attributes
// (it doesn't apply in main.rs for example)

// this is an enum.
// we can "enumerate" all these possible variants, which is where enumeration gets its name.
enum IpAddrKind {
    V4,
    V6,
}
// IP addresses are either version four or version six addresses,
// but not both at the same time.
// That property of IP addresses makes the enum data structure appropriate

fn route(ip_kind: IpAddrKind) {}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // we can call the function with both variants,
    // since they are of type `IpAddrKind`
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hey"));
    m.call();

    // Rust doesnt have null, but an Option enum
    // the Some variant of the Option enum can hold any piece of data of any type
    // (because of the <T> generic)
    // this is the definition:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("hi");

    // if we have nothing, we need to specify the type
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this wouldn't work now:
    //    let sum = x + y;
    // we need to handle the different cases first
    // we have to convert an Option<T> to a T before we can perform T operations with it

    println!("value: {}", value_in_cents(Coin::Penny));
    println!("value: {}", value_in_cents(Coin::Dime));
    println!("value: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u16_value = 0u16;
    match some_u16_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // _ matches any value
        _ => (),
        // as a result we can say that we want to do nothing for all values we didnt list previously
    }

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // ^^^^
    // this is the
    // same as this
    //         vvvv
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // it's syntax sugar
}

// file://///wsl$/Ubuntu/home/pengin/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch06-03-if-let.html

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // if we don't specify the case of None, it would error
        // because
        None => None,
        // and here we obtain the specified number
        Some(i) => Some(i + 1),
    }
}
// by the way,
// matches in Rust are exhaustive:
// we must exhaust every last possibility in order for the code to be valid

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we are also able to define methods on enums, just like for structs!
impl Message {
    fn call(&self) {
        //method body
    }
}

#[derive(Debug)] // for easy debugging
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,  // this is an arm
        Coin::Nickel => 5, // we have 4 arms here
        Coin::Dime => 10,
        // we can obtain the specified state too
        Coin::Quarter(state) => {
            // we can also have code
            println!("a state quarter from: {:?}", state);
            25
        }
    }
}
