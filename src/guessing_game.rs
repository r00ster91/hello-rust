use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process::exit;

fn check(size: &str, attempts: u32, secret_number: u32) {
    if attempts == 0 {
        println!("Too {}! Game over! The secret number was {}", size, secret_number);
        exit(0);
    }
    let plural = if attempts == 1 { "" } else { "s" };
    println!("Too {}! {} attempt{} left", size, attempts, plural);
}

pub fn main() {
    let mut attempts = 5;

    println!(
        "Guess the secret number ranging from 1 to 100!\n\
              You have {} attempts.",
        attempts
    );

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        print!("Guess: ");

        // print doesn't flush on its own
        io::stdout().flush().expect("Unable to flush");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                attempts-=1;
                check("small", attempts, secret_number);
            },
            Ordering::Greater => {
                attempts-=1;
                check("big", attempts, secret_number);
            },
            Ordering::Equal => {
                println!("Correct! Congratulations.");
                break;
            }
        }
    }
}
