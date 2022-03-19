#![allow(unused)]

pub fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // let x = 5;
    // is
    // let PATTERN = EXPRESSION;

    let (x, y, z) = (1, 2, 3);

    //                   THIS is a pattern too!
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // refutable pattern
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // irrefutable pattern (gives warning)
    if let x = 5 {
        println!("{}", x);
    };

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // this matches!
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let var = 5;

    match var {
        5 | 10 => println!("{:?} turns out to be 5 or 10!", var),
        _ => println!("nothing"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // this is destructing a struct!
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // since THIS pattern matches, it works
    // this is the field shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // of course we can use that kind of pattern in `match` too
    match p {
        Point { x, y: 8 } => println!("y is 8"),
        Point { x: 0, y } => println!("x is 0 for sure!"),
        Point { x, y } => println!("anything else. This always matches!"),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // patterns can be really deep!
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    fn hello(_: i32, y: i32) {
        println!("only y");
    }

    hello(9, 6);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let something = Some(10);
    match something {
        Some(x) if x > 5 => println!("bigger than 5!"),
        Some(x) => println!("any other something"),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("certainly!"),
        _ => println!("no"),
    }

    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_variable @ 3..=7 } => {
            // if we still want to access the value we matched in here, we use @ as above
            println!("Found an id in range: {}", id_variable)
        },
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
