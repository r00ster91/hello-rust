// since these are the owned type `String`, the instances of this struct will own all its data,
// no references
//
// if we want references, we need to specify lifetimes which we dont know about yet
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn new_user(name: String, email: String) -> User {
    User {
        // rather than `name: name` and so on, we can simply write it like this!
        // it's the field init shorthand syntax
        name,
        email,
        active: true,
        sign_in_count: 1,
    }
}

pub fn main() {
    // if we want to modify anything, the whole instance needs to be marked as mutable
    let mut user1 = User {
        email: String::from("mrguy@guynet.com"),
        name: String::from("guy129"),
        active: true,
        sign_in_count: 1,
    };

    user1.name.push_str("3");

    user1.sign_in_count += 5;

    println!("{}: {}", user1.name, user1.email);

    let user0 = new_user(String::from("hi"), String::from("hey"));

    println!("{}", user0.name);

    // It's often useful to create a new instance of a struct that uses most of an old instance's
    // values but changes some
    // for this, theres the struct update syntax:
    let user2 = User {
        email: String::from("person@personland.com"),
        name: String::from("anotherpersonxx23xx"),
        ..user1 // all others stay the same
    };

    println!("{}", user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    //variables
    let w = 50;
    let h = 50;
    println!("Area is {}", area1(w, h));

    let rect1 = (50, 50);
    println!("Area is {}", area2(rect1));

    let rect2 = Rect {
        width: 50,
        height: 50,
    };
    println!("Area is {}", area3(&rect2));

    // For pretty-printing:
    println!("Rect = {:#?}", rect2);
    // Other one:
    println!("Rect = {:?}", rect2);

    //methods are different from functions in that
    //they're defined within the context of a struct (or an enum or a trait object)
    println!("Area is {}", rect2.area());
    // having methods like this is really good for organisation and libraries and stuff

    //these two are the same:
    rect2.area();
    (&rect2).area();
    // it's called "automatic referencing and dereferencing"

    let r1 = Rect {
        width: 30,
        height: 50,
    };
    let r2 = Rect {
        width: 10,
        height: 40,
    };
    let r3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can r1 hold r3? {}", r1.can_hold(&r3));
    println!("Can r1 hold r2? {}", r2.can_hold(&r2));

    // to call an associated function (NOT a "method"),
    // we need to use :: instead of . to call the function.
    let square = Rect::square(50);

    println!("{:?}", square);
    // the :: syntax is used for both associated functions and namespaces created by modules
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rect) -> u32 {
    rect.width * rect.height
}

// this derives the Debug "trait" in order to format this struct
#[derive(Debug)] // this is an annotation
struct Rect {
    width: u32,
    height: u32,
}

// let's define a method inside this `impl` (implementation) block
impl Rect {
    // the first parameter is always `self`, which represents the instance of the struct the method is being called on
    // we choose to borrow immutably here as we simply want to read from it, not write
    // we dont want ownership.
    //
    // we take ownership if the method transforms `self` into something else
    // and you want to prevent the caller from using the original instance after the transformation.
    fn area(&self) -> u32 {
        &self.width * &self.height
    }

    // if we use `self` and it's a method, other arguments MUST always follow after `&self`
    fn can_hold(&self, other_rect: &Rect) -> bool {
        self.width >= other_rect.width && self.height >= other_rect.height
    }
}

// there's no special reason here to define this function separately with a new `impl Rect {`,
// but there is cases where this syntax is useful too
impl Rect {
    // this, however, is an Associated Function because it's just associated with the struct
    // they are functions, NOT methods because they don't have an instance of the struct to work with!
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size
        }
    }
}

// tuple struct without named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
