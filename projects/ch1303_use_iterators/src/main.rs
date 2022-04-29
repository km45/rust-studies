extern crate ch1303_use_iterators;

use std::env;
use std::process;

use ch1303_use_iterators::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
