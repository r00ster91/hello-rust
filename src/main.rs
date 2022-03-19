mod collections;
mod concurrency;
mod enums;
mod error_handling;
mod fibonacci;
mod functional;
mod generics_and_traits;
mod guessing_game;
mod lifetimes;
mod miscellaneous;
mod ownership;
mod patterns;
mod smart_pointers;
mod structs;

fn main() {
    guessing_game::main();
    miscellaneous::main();
    fibonacci::main();
    ownership::main();
    structs::main();
    enums::main();
    collections::main();
    error_handling::main();
    generics_and_traits::main();
    lifetimes::main();
    functional::main();
    smart_pointers::main();
    concurrency::main();
    patterns::main();
}
