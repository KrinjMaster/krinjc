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

fn run_interpreter() -> Result<(), CError> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_promting(),
        2 => run_from_file(&args[1].clone()),
        _ => return Err(CError::Input("Incorrect number of arguements!".to_string())),
    }
}

fn main() {
    match run_interpreter() {
        Ok(()) => {}
        Err(err) => handle_err(err),
    }
}
