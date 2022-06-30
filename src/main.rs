// all clippy warnings
#![warn(clippy::all)]

mod common;
mod engine;
mod error;
mod parse;

// re-export
pub use common::*;
pub use engine::PaymentsEngine;
pub use error::Error;

// wrapper function to print the error message using Display instead of Debug
fn main() {
    match engine::run() {
        Ok(()) => {}
        Err(e) => {
            // print error message
            eprintln!("{}", e);
            // exit with code 1 to signify an error
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests;
