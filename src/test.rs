use krinjc::*;

use crate::run_interpreter;

// incorrect number of arguements
#[test]
fn number_of_args() {
    let err = run_interpreter(Some(vec!["run".into(), "odin".into(), "dva".into()])).unwrap_err();

    match err {
        CError::Input(msg) => {
            assert_eq!(msg, "Incorrect number of arguments");
        }
        _ => panic!("Expected Input error"),
    }
}

// file does not exist
#[test]
fn file_doesnt_exist() {
    let err = run_interpreter(Some(vec!["run".into(), "/src/abracadabra".into()])).unwrap_err();

    match err {
        CError::Context { message, source } => {
            assert_eq!(message, "Failed to open file");
            assert!(source.to_string().contains("No such file or directory"));
        }
        _ => panic!("Expected Context error"),
    }
}

// incorrect file extension
#[test]
fn incorrect_extension() {
    let err = run_interpreter(Some(vec![
        "run".into(),
        "./src/test/incorrect_extension.cpp".into(),
    ]))
    .unwrap_err();

    match err {
        CError::Input(msg) => {
            assert_eq!(msg, "Incorrect or emtpy file extension, use `.krj`");
        }
        _ => panic!("Expected Input error"),
    }
}

// reading directory, instea of file
#[test]
fn directory_not_file() {
    let err = run_interpreter(Some(vec!["run".into(), "./src/test/".into()])).unwrap_err();

    match err {
        CError::Input(msg) => {
            assert_eq!(msg, "Path is a directory");
        }
        _ => panic!("Expected Input error"),
    }
}

// empty file with correct extension
#[test]
fn empty_file() {
    let diagnostics =
        run_interpreter(Some(vec!["run".into(), "./src/test/empty_code.krj".into()])).unwrap();

    assert!(diagnostics.has_errors());

    match &diagnostics.errors[0] {
        CError::Compile(msg) => {
            assert_eq!(msg, "Empty source code");
        }
        _ => panic!("Expected Compile error"),
    }
}

// correct file with compile token error
#[test]
fn incorrect_token() {
    let diagnostics = run_interpreter(Some(vec![
        "run".into(),
        "./src/test/incorrect_token.krj".into(),
    ]))
    .unwrap();

    assert!(diagnostics.has_errors());

    assert!(
        diagnostics
            .errors
            .iter()
            .any(|e| matches!(e, CError::Compile(_))),
        "Expected at least one Compile error"
    );
}

//unterminated string
#[test]
fn unterminated_string() {
    let diagnostics = run_interpreter(Some(vec![
        "run".into(),
        "./src/test/unterminated_string.krj".into(),
    ]))
    .unwrap();

    assert!(diagnostics.has_errors());

    assert!(
        diagnostics
            .errors
            .iter()
            .any(|e| matches!(e, CError::Compile(_))),
        "Unterminated string"
    );
}
