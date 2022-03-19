mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}


// instead of these two lines below, we can also simply use `use`, like
      use self::front_of_house::hosting; // this is a relative path
// after that we could do
//      hosting::add_to_waitlist();
//
// we can also do
//      pub use self::front_of_house::hosting;
// this is called reexporting because now we have this available for us
// and it's also public in external code now

pub fn eat_at_restaurant() {
    // this is an absolute path
    //      crate::front_of_house::front_of_house::hosting::add_to_waitlist();

    // this is a relative path
    //      front_of_house::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // as an example, compare it to this:
    //     Linux absolute path:
    //     /home/users/c/computerhope/public_html/cgi-bin
    //     Linux relative path:
    //     /public_html/cgi-bin

    let mut meal = back_of_house::Breakfast::summer("Rye");

    // now, for an instance, we use dot notation
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // we can access all members of the Appetizer enum with just a single `pub` in front of `enum`
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}

use std::collections::HashMap;

// using `as`, we can create aliases for types
use std::{fmt::Result, io::Result as IoResult};
// and using this bracket syntax we can also take in multiple types at once

use std::io::{self, Write};
// this is the same as
//   use std::io;
//   use std::io::Write;

fn function1() /*-> fmt::Result*/ {/*...*/}
fn function2() /*-> IoResult<()>*/ {/*...*/}

//the glob operator exists too:
use std::collections::*;
// this imports all stuff of std::collections
