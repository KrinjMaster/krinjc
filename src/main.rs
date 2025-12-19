mod utils;

use std::env;
// use std::io;
use utils::{handle_err, Error};

fn run_interpreter() -> Result<(), Error> {
    let input: Vec<String> = env::args().collect();

    match input.len() {
        1 => println!("Prompt executing"),
        2 => println!("Compiling file"),
        _ => {
            return Err(Error::InputError(
                "Incorrect number of arguements!".to_string(),
            ))
        }
    }

    Ok(())
}

fn main() {
    match run_interpreter() {
        Ok(()) => {}
        Err(err) => handle_err(err),
    };
}
