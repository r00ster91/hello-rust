pub mod hosting;

// src/main.rs and src/lib.rs are called the crate roots
// Cargo passes the crate root files to rustc to build the library or binary
pub mod front_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // with this `super` we can go back by one. it's like `cd ..`
        super::serve_order();
    }
    fn cook_order() {}
}

fn serve_order() {}
