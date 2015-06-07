
extern crate cratesandmodules;

use cratesandmodules::english::{greetings, farewells};
use cratesandmodules::russian;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Russian: {}", russian::hello());
    println!("Goodbye in Russian: {}", russian::goodbye());
}

