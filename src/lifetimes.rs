// The concept of lifetimes is somewhat different from tools in other programming languages,
// arguably making lifetimes Rust’s most distinctive feature
// The main aim of lifetimes is to prevent dangling references,
// which cause a program to reference data other than the data it’s intended to reference
pub fn main() {
    let x = 5;

    let y = &x;

    println!("{}", y);

    let string1 = String::from("hi"); // note that this could also just be a literal
    let string2 = "xyz";

    println!("longest: {}", longest(string1.as_str(), string2));

    // one special lifetime is `'static`, which means that this reference can live for
    // the entire duration of the program.
    // All string literals have the `'static` lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    // The text of this string is stored directly in the program’s binary, which is always available.
    // Therefore, the lifetime of all string literals is `'static`.
}

// if they all have the same lifetime, it works
// the default parameter name is `a`
// btw `<'a>` is what actually DECLARES/introduces/names the lifetime!
fn longest<'b, 'a>(x: &'a str, y: &'a str) -> &'a str {
// <'b, 'a> is how we can have multiple lifetimes if we ever need that, but here we dont need it
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// these lifetime parameters are not changing a lifetime, but simply annotating them
// so that rust can enforce them

// rust can analyze the code within the function without any help, but
// when a function has references to or from code outside that function,
// it becomes almost impossible for Rust to figure out the lifetimes of the parameters
// or return values on its own
// The lifetimes might be different each time the function is called!
//  This is why we need to annotate the lifetimes manually

// structs can hold references if we specify lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str,
}
