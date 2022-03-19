use ferris_says::say;
use rand::Rng;
use std::env;
use std::io;
use std::io::{stdout, BufWriter, Write};
use std::process::exit;

fn nine() -> u32 {
    9
}
/*
hopefully, this comment
will explain what's going on
*/
pub fn main() {
    // This is the lyrics to the Christmas carol "The Twelve Days of Christmas"
    // it comes first in the program because the output of it is so long
    let days = ["first", "second", "third",
                "fourth", "sixth", "seventh",
                "eighth", "ninth", "tenth",
                "eleventh", "twelfth"];
    let items = ["a partridge in a pear tree",
                 "two turtle doves",
                 "three french hens",
                "four calling birds",
                "five gold rings",
                "six geese a-laying",
                "seven swans a-swimming",
                "eight maids a-milking",
                "nine ladies dancing",
                "ten lords a-leaping",
                "eleven pipers piping",
                "twelve drummers drumming"];
    // this iteration still isn't really like on
    // https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)#/media/File:12_days_angus.png

    // 0..=12 is an inclusive range, it includes 12
    for (day_count, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", day);
        for (i, item) in items.iter().enumerate() {
            println!("{}", item);
            if day_count == i {
                break
            }
        }
    }

    //A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    let compound_type_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = compound_type_tup;
    println!("Y: {}", y);

    let mut array_size = 5;
    let array = [3; 5/*can't use array_size here*/];

    // println!("{:?}", array);

    while array_size > 0 {
        array_size -= 1;
        println!(
            "Array value at index {} is {}",
            array_size, array[array_size]
        );
    }

    println!("Number {}", nine());

    let another_array = [9, 3, 2, 3, 4, 5];

    for value in another_array.iter() {
        println!("another_array's value is {}", value);
    }

    // 1..=10 is an INclusive range, unlike 1..10 which would start
    // counting from 9 instead.
    for countdown in (1..=10).rev() {
        println!("LAUNCHING IN {}", countdown);
    }

    let stdout = stdout();
    let message = "i am very powerful";

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), message.len(), &mut writer).unwrap();

    // Interestingly, here `return;` will flush and output Ferris,
    // but `exit(0);` would NOT flush and does not output Ferris.

    // Here, first of all we have to flush, because other stuff follows after this,
    // so the program isn't automatically flushed on exit, but
    // we can not flush `stdout()` here, instead we need to flush `writer`!
    writer.flush().expect("hello");

    let mut index = 0;
    loop {
        index += 1;
        if index != 20 {
            continue;
        } else {
            break;
        }
    }
    println!("{}", index);

    while index != 40 {
        index += 1
    }
    println!("{}", index);

    let random_number = rand::thread_rng().gen_range(0, 10);

    if random_number > 5 {
        println!("{} is bigger than five!", random_number);
    } else if random_number == 0 {
        println!("{} is zero!", random_number);
    } else {
        println!("{} is smaller than five!", random_number);
    }

    print!(
        "1. Convert Fahrenheit to Celsius\n\
         2. Convert Celsius to Fahrenheit\n\
         Input: "
    );
    io::stdout().flush().expect("unable to flush");

    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).expect("unable to read");

        let number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                input.clear();
                continue;
            }
        };

        if number == 1 {
            print!("Fahrenheit: ");
            io::stdout().flush().expect("cant flush");
            loop {
                input.clear();

                io::stdin().read_line(&mut input).expect("cant read");

                let number: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Celsius: {}", (number - 32.) / 1.8);
            }
        } else if number == 2 {
            print!("Celsius: ");
            io::stdout().flush().expect("cant flush");
            loop {
                input.clear();

                io::stdin().read_line(&mut input).expect("cant read");

                let number: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Fahrenheit: {}", (number * 1.8) + 32.);
            }
        }
    }
}
