use krinjc::CError;

pub fn handle_err(err: CError) {
    eprintln!("ERROR {}\n", err);

    eprintln!("run 'jkrinjc --help' to view all commands");
}
