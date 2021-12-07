mod back_of_house;
mod front_of_house;

// Usually not use full path to bring paths for functions.
use crate::front_of_house::hosting;

fn serve_order() {}

// Use `pub use` for re-exporting.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    self::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    // `fn eat_at_restaurant` can access `mod front_of_house` though
    //  `front_of_house` is not public because `crate::eat_at_restaurant()`
    // and `crate::front_of_house` are in same module `crate`.

    hosting::add_to_waitlist();

    // ========================================

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    // ========================================
    // Usually use full path to bring paths for structs and enum.
    use crate::back_of_house::Appetizer;
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}

use std::fmt::Result;
use std::io::Result as IoResult; // give new name with `as`

// Bring multiple paths using `{}`.
use std::io::{self, Write};
use std::{array, cmp::Ordering};

// glob operator `*`
use std::collections::*;

fn main() {}
