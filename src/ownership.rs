pub fn main() {
    // `hello` is only useable within this scope
    {
        // compared to a string literal, here it's allocated on the heap
        // it needs to be on the heap because we modify the string
        // on the stack the size needs to be known, on the heap it doesn't
        let mut hello = String::from("hello");
        hello.push_str(" world");
        println!("{}", hello); // non-literal needs to be formatted
    }
    // here Rust called `drop` and so `hello` is deallocated

    let s1 = String::from("hi");
    let s2 = s1; // s1 was moved into s2
                 // s1 was invalidated
    // it's good to note that no new memory was allocated here

    //println!("{}", s1);// wouldn't work
    println!("{}", s2);

    // However, we can explicitly clone, i.e. allocate new, memory
    let s3 = s2.clone();
    //both works:
    println!("s2 = {}, s3 = {}", s2, s3);

    let string = String::from("안녕");
    take_string(string);
    // here, `string` is invalidated
    // that means we can not borrow it.
    // we cant borrow after a move

    let x = 5;
    use_integer(x);
    // here, `x` is still valid
    //
    // it's because x doesn't require allocation anyway,
    // so disallowing to copy such a scalar value would only be a hindrance


    let (another_string, sixsixsix) = allocate_str();

    // we can use it
    println!("we got {} and {}", another_string, sixsixsix);

    let move_into_here = take_and_give_back(another_string);

    // we can still use it, but not `another_string` anymore
    println!("{}", move_into_here);


    // But now, we can do it even simpler.
    // We can simply use references that allow us to
    // refer to some value WITHOUT taking ownership of it!
    let what_is_the_size_of_this = String::from("how long am i?");

    // having references as function parameters is called "borrowing"
    println!("the length of \"{}\" is {}", what_is_the_size_of_this, length(&what_is_the_size_of_this));

    // & is referencing
    // * is the opposite: dereferencing


    // we can borrow value as mutable, but only once, in order to prevent data races
    let mut s = String::from("hello");

    let r1 = &mut s;
    // this would error:
    //let r2 = &mut s;

    //println!("{}, {}", r1, r2);
    println!("{}", r1);

    // multiple immutable references are okay however because no one who is just reading
    // the data has the ability to affect anyone else's reading of the data

    // There is something called string slices.
    let hw = String::from("welcome to this world");
    // they are references to specific parts of a string
    let hello = &hw[..5];//same as 0..5
    let world = &hw[6..]; //same as 6..hw.len()
    let whole_string = &hw[..]; //same as 0..hw.len()
    // no memory is cloned or allocated here

    println!("the first word is: '{}'", first_word(&hw));

    let literal = "i am literally a literal";
    // the type of `literal` here is &str.
    // It's a slice pointing to that specific point of the binary


    let a = [1, 2, 3, 4, 5];
    // this is also a slice of type &[i32]
    let slice = &a[1..3];
}

// The type that signifies "string slice" is written as `&str`
// Instead of &String we use &str for the parameter because
// if we have a String, we can pass a slice of the entire String.
// Defining a function to take a string slice instead of a reference to a String
// makes our API more general and useful without losing any functionality:
fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    // we are passed a reference so we need to write `&byte`
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // with the exclusive range we don't include the space in the result
            return &string[..index];
        }
    }

    &string[..]
}

// Rust also prevents dangling references
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
//     // here, it's cleaned up, so its memory goes away
//     // danger!
// }
// Rust wouldn't let it compile.
// We can simply solve this by not returning a reference,
// but returning the String directly (as `s`)

// I think it's a good convention to generally use usize as the integer type
fn length(string: &String) -> usize {
    string.len()
} // here, `goes` out of scope, but only the reference to it
// also, we can't modify references, they are immutable by default



fn take_string(string: String) {
    println!("{}", string);
} // now your string is gone and deallocated

fn use_integer(x: u32) {
    // basically, we always need to be format, unless it is a STRING (and ONLY string) literal
    println!("{}", x);
} // x goes out of scope and nothing special happens

fn allocate_str() -> (String, u32) {
    let something = String::from("your string");
    (something, 666) // this is how we can return multiple values at once!

    // here, the string/memory is NOT `drop`ed/freed/deallocated because
    // we return it to outside the function!
}

fn take_and_give_back(string: String) -> String {
    string
}
