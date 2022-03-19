pub fn main() {
    // if it starts off as empty, we need to specify the type it holds
    // it can only store one type
    let v: Vec<i32> = Vec::new();

    // here, we dont need a type annotation because it's inferred by rust as i32,
    // the default integer type
    let v2 = vec![1, 2, 3];

    // here, however, we dont need a type annotation due to the code that follows
    let mut v3 = Vec::new();

    // now, let's update the vector with some stuff
    v3.push(5);
    v3.push(10);
    v3.push(15);
    v3.push(20);
    v3.push(20);

    // let's read it!
    let third: &i32 = &v3[3];
    println!("third element is: {}", third);

    match v.get(2) {
        Some(third) => println!("third element is: {}", third),
        None => println!("jokes on you! there is no third element"),
    }

    // theres a difference between `get` and `[]` though
    // this would panic:
    //      let non_existent1 = &v3[100];
    // but this, returns a None!
    let non_existent2 = v3.get(100);

    let first = &v3[0];

    // this doesnt work:
    //      v3.push(11);
    // we cant borrow as mutable because it is also borrowed as immutable which doesnt work because
    // adding a new element onto the end of the vector might require
    // allocating new memory and copying the old elements to a new space,
    // if there isn’t enough room to put all the elements next to each other
    // where the vector currently is.

    println!("first element is {}", first);

    let mut vv = vec![1, 3, 3, 2, 1000];

    for value in &vv {
        println!("{}", value);
    }

    // now let's change the values while iterating
    for value in &mut vv {
        // to update, we need to dereference!
        *value = *value + 50;
        // it's because above we got a mutable reference, and we need to dereference
        // that reference first to really modify that value

        println!("{}", value);
    }

    // now, what if we want to have different types in the same vector?
    // we can use an enum because it's one type, but inside we can have many different
    // types!
    #[derive(Debug)]
    enum SpreadsheetCall {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetCall::Int(50),
        SpreadsheetCall::Float(12.123),
        SpreadsheetCall::Text(String::from("hello!")),
    ];

    let popped_element = row.pop();
    println!("present: {:#?}\npopped: {:?}", row, popped_element);

    // Rust has only one string type in the core language,
    // which is the string slice `str` that is usually seen in its borrowed form `&str`
    // The reason it's usually seen as &str is because that is a reference to the point in
    // binary where the string is. it just makes little sense to use `str` directly like that

    // this, also, is allocated on the heap
    // it's dynamically growable
    let empty_string = String::new();

    let string1 = "hello".to_string();
    // this is the same as
    let string2 = String::from("hello");

    let non_ascii_string = String::from("안녕하세요");

    // we can NOT index into strings like string[0] because it is simply not clear
    // what should be returned for that operation!

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // but we can still use s2 because we only specified a reference to it
                       // and + only takes a reference as the "second argument"

    // now, what if we want to concatenate a bunch of stuff?
    // we can do this:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s10 = s1 + "-" + &s2 + "-" + &s3;
    // but with all of the + and " characters, it's difficult to see what's going on!
    // it's unwieldy so let's use `format!` for this instead!

    let s11 = String::from("tic");
    let s22 = String::from("tac");
    let s33 = String::from("toe");
    let s20 = format!("{}-{}-{}", s11, s22, s33);
    // this does the same as before and doesn't take ownership of any arguments either

    println!("s10: {}, s20: {}", s10, s20);

    println!("s3: {}, s2: {}", s3, s2);

    // but this works:
    println!("{}", "नमस्ते".chars().nth(1).unwrap());

    // now, hash maps!
    // let's store the score of a teams in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("red"), 50);
    scores.insert(String::from("blue"), 30);
    // because we insert these keys and values, rust can infer the types

    // just like vectors, hash maps are homogeneous,
    // so all keys have the same type and all values have the same type

    // as with all collections, it stores its data on the heap
    // it's of course they are all dynamic and growable

    // we can also zip two vectors together to a hash map
    let teams1 = vec![String::from("red"), String::from("blue")];
    let initial_scores = vec![50, 30];

    // the type annotation HashMap<_, _> is required here because `collect`
    // can collect into many different data structures and it can't know
    //
    // the types in this HashMap can however be type-inferred and as such
    // we can make it _, _
    // it will be inferred as String and i32
    let scores1: HashMap<_, _> = teams1.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point!
    // the `map` has basically ABSORBED them!

    // now let's read a value with a key
    println!("red: {}", scores.get("red").unwrap());
    // just like for vectors, we can access the data structure
    // using `get`, but it will return an Option which we will have to handle
    // accordingly

    // and in a similar manner to vectors, we can iterate through it
    for (key, value) in &scores1 {
        println!("value: {}, key: {}", value, key);
    }
    // the order is always key and then value
    // note that it is printed in an arbitrary order

    println!("{:?}", map);
    // to replace a value, we simply call `insert` again
    map.insert(String::from("Favorite color"), String::from("green"));
    println!("{:?}", map);

    // if we only want to replace/insert if the value isn't there, we can do this:
    let mut scores5 = HashMap::new();
    scores5.insert(String::from("Blue"), 10);

    scores5.entry(String::from("Yellow")).or_insert(50);

    // this will do nothing because Blue already exists
    scores5.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores5);

    // let's count the occurrences of all words in a text
    let text = "hello hello world hi hi hello no no yes here is a text that is repeating some words and word";

    let mut word_tracker = HashMap::new();

    for word in text.split_whitespace() {
        // this checks whether this word is already in the tracker and if not
        // it inserts it and also sets it to zero
        let count = word_tracker.entry(word).or_insert(0);
        // and here we dereference the mutable reference we got to add 1 to the entry
        *count += 1;
    }

    println!("{:?}", word_tracker);

    // lastly, by default, HashMap uses a "cryptographically strong" hashing function
    // that can provide resistance to Denial of Service (DoS) attacks
    // this is not the fastest and you can specify a different hasher.
    // you can download such other haser implementations from crates.io

    let mut integers = vec![
        23, 2, 3, 23, 23, 32, 32, 3, 2, 32, 3, 23, 232, 2232323, 23, 23, 2, 32, 3, 2, 3, 23, 2,
        333333322, 222, 322, 23, 1, 2, 3, 3, 3, 4, 5, 1, 2, 2, 2, 3, 3, 3, 100, 2302, 2320,
    ];
    // mean (average):
    let mut mean = 0;
    // to avoid moving into the for loop we use `&integers`
    for integer in &integers {
        mean += integer;
    }
    // btw: we could also use &integers.iter().sum()
    // with that `&integers` we can then still use `integers` here too
    mean /= &integers.len();
    println!("Mean: {}", mean);

    // median (middle position when sorted):
    integers.sort();
    let median = integers[integers.len() / 2];
    println!("Median: {}", median);

    // mode (value that occurs the most):
    let mut tracker = HashMap::new();
    for integer in &integers {
        let count = tracker.entry(integer).or_insert(0);
        *count += 1;
    }
    let mut most_present_key = 0;
    let mut occurrences = 0;
    for (key, value) in &tracker {
        if value > &occurrences {
            most_present_key = **key;
            occurrences = *value;
        }
    }
    println!("Mode: {}", most_present_key);

    println!("converted: {}", pig_latin("first apple"));

    let mut company: HashMap<String, String> = HashMap::new();
    let mut input = String::with_capacity(2);
    loop {
        print!("Quick&Slick Company register\n1: Add\n2: Retrieve\n> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let chr = input.chars().next().unwrap();
        if chr == '1' {
            print!("To add an employee to a department, input as follows: `{{employee name}} {{department}}`\n> ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            let split: Vec<&str> = input.split(' ').collect();
            let employee_name = split[0].to_string();
            let department = split[1].trim().to_string();
            company.insert(employee_name, department);
        } else if chr == '2' {
            let mut keys = company.keys().collect::<Vec<&String>>();
            keys.sort();
            println!("Employees:");
            for key in keys.iter() {
                println!("{}: {}", key, company[*key]);
            }
        }
        input.clear();
    }
}

use std::io;
use std::io::Write;

use std::collections::HashMap;

fn pig_latin(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    for word in string.split_whitespace() {
        let chr = word.chars().next();
        if chr.is_some() {
            match chr.unwrap() {
                'a' | 'i' | 'u' | 'e' | 'o' => {
                    // Vowel
                    result.push_str(word);
                    result.push_str("-hay");
                }
                _ => {
                    // Consonant
                    result.push_str(&word[1..]);
                    result.push('-');
                    result.push(chr.unwrap());
                    result.push_str("ay");
                }
            }
            result.push(' ');
        }
    }
    result
}
