#![forbid(unsafe_code)]
// -----------------------------------------
//
//  My own implementation of Lox interpreter
//  from the book "Crafting Interpreters" by
//  Robert Nystrom.
//
// -----------------------------------------
mod utils;

use krinjc::{run_from_file, run_promting, CError, Diagnostics};
use std::env::args;
use utils::handle_errs;

// added arguements for testing, see test.rs for more
fn run_interpreter(arguments: Option<Vec<String>>) -> Result<Diagnostics, CError> {
    let args = arguments.unwrap_or_else(|| args().collect());

    match args.len() {
        1 => Ok(run_promting()?),
        2 => run_from_file(&args[1]),
        _ => Err(CError::Input(format!("Incorrect number of arguments"))),
    }
}

fn main() {
    match run_interpreter(None) {
        Err(err) => {
            handle_errs(vec![err]);
            std::process::exit(1);
        }
        Ok(diag) => {
            if diag.has_errors() {
                handle_errs(diag.errors);
                std::process::exit(65);
            }
        }
    }
}

#[cfg(test)]
mod test;
