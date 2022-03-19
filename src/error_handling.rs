#![allow(unused)]
pub fn main() {
    //panic!("oh no i've failed!");
    let v = vec![1, 2, 3];

    //v[999];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("help, fire! {:?}", e)
            },
            other_error => {
                panic!("some fire's here! {:?}", other_error)
            }
        }
    };

    // we can write that code like this too!
    let ff = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::convert::TryInto;
use std::process::exit;

fn read_username() -> Result<String, io::Error> {
    let mut file = match File::open("username.txt") {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut string = String::with_capacity(
        file.metadata().unwrap().len().try_into().unwrap_or_else(|_| {
            println!("File is too big!");
            exit(1);
        })
    );

    match file.read_to_string(&mut string) {
        Ok(_) => return Ok(string),
        Err(err) => return Err(err)
    }

    //instead of all THAT
    // we can also do this:
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    // the ? thing returns an Err if it fails automatically!

    Ok(s)
}
