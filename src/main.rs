// -----------------------------------------
//
//  My own implementation of Lox interpreter
//  from the book "Crafting Interpreters" by
//  Robert Nystrom.
//
// -----------------------------------------
mod utils;

use krinjc::{run_from_file, run_promting, CError};
use std::env::args;
use utils::handle_err;

// added arguements for testing, see test.rs for more
fn run_interpreter(arguements: Option<&[String]>) -> Result<(), CError> {
    let arges = match arguements {
        Some(a) => a,
        None => &args().collect::<Vec<_>>(),
    };

    match arges.len() {
        1 => run_promting(),
        2 => run_from_file(&arges[1].clone()),
        _ => return Err(CError::Input("Incorrect number of arguements".to_string())),
    }
}

fn main() {
    if let Err(err) = run_interpreter(None) {
        handle_err(err);
    }
}

#[cfg(test)]
mod test;
