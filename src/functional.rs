#![allow(unused)]

use std::{thread, time::Duration};

// now onto Functional Language Features: Iterators and Closures!
// I think the reason they are called *function*al features is
// because closures are *function*-like

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("doing some very expensive workout calculations now...");
    thread::sleep(Duration::from_secs(2));
    intensity // done!
}

fn generate_workout(intensity: u32, random_number: u32) {
    // explicitly type-annotated for demonstration purposes
    let mut expensive_result = Cacher::new(|param1: u32| -> u32 {
        println!("doing some very expensive workout calculations now...");
        thread::sleep(Duration::from_secs(2));
        param1 // done!
    }); // semicolon to complete the `let` statement

    if intensity < 25 {
        println!("Today, do {} pushups!",expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;

    let equal_to_x = |z: i32| -> bool { z == x };

    let y = 4;

    assert!(equal_to_x(y));

    let v = vec![1, 2, 3];

    println!("sum: {}", v.iter().sum::<i32>());

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // because iterators and do nothing unless consumed, this'd not do the same as above:
    // let v2: Vec<_> = v1.iter().map(|x| x + 1);

    assert_eq!(v2, vec![2, 3, 4]);

    let vector = vec![32, 32,31, 2, 3,1, 2, 2,1 , 1, 2, 3, 5, 6, 1, 10, 20, 20, 30,23];

    println!("{:?}", vector.into_iter().filter(|x| x > &3).collect::<Vec<i32>>());

    // our own custom iterator!
    let mut counter = Counter::new();

    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// now we'll create a struct that will hold the closure and the resulting value of calling the closure
// The struct will execute the closure only if we need the resulting value,
// and it will cache the resulting value so the rest of our code doesn't have to be
// responsible for saving and reusing the result.
// This pattern is called "memoization" or "lazy evaluation".

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

