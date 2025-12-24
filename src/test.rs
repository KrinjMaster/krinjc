use krinjc::*;

use crate::run_interpreter;

// incorrect number of args
#[test]
fn number_of_args() {
    match run_interpreter(Some(&vec![
        "run".to_string(),
        "odin".to_string(),
        "dva".to_string(),
    ])) {
        Err(CError::Input(msg)) => {
            assert_eq!(msg, "Incorrect number of arguements".to_string())
        }
        _ => panic!("Incorrect error message!"),
    }
}

// file does not exist
#[test]
fn file_doesnt_exist() {
    match run_interpreter(Some(&vec![
        "run".to_string(),
        "/src/abracadabra".to_string(),
    ])) {
        Err(CError::Context { message, source }) => {
            assert_eq!(message, "Failed to open file".to_string());
            assert!(source
                .to_string()
                .contains("No such file or directory (os error 2)"));
        }
        _ => panic!("Incorrect error message!"),
    }
}

// incorrect file extension
#[test]
fn incorrect_extension() {
    match run_interpreter(Some(&vec![
        "run".to_string(),
        "./src/test/incorrect_extension.cpp".to_string(),
    ])) {
        Err(CError::Input(message)) => {
            assert_eq!(
                message,
                "Incorrect or emtpy file extension, use `.krj`".to_string()
            );
        }
        _ => panic!("Incorrect error message!"),
    }
}

// reading directory, instea of file
#[test]
fn directory_not_file() {
    match run_interpreter(Some(&vec!["run".to_string(), "./src/test/".to_string()])) {
        Err(CError::Input(message)) => {
            assert_eq!(message, "Path is a directory".to_string());
        }
        _ => panic!("Incorrect error message!"),
    }
}

// empty file with correct extension
#[test]
fn empty_file() {
    match run_interpreter(Some(&vec![
        "run".to_string(),
        "./src/test/empty_code.krj".to_string(),
    ])) {
        Err(CError::Compile(message)) => {
            assert_eq!(message, "Empty source code".to_string());
        }
        _ => panic!("Incorrect error message!"),
    }
}
