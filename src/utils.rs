use colored_text::Colorize;
use krinjc::CError;

pub fn handle_errs(errors: Vec<CError>) {
    for err in &errors {
        eprintln!("{} {}\n", "ERROR".red().bold(), err.to_string().bold());
    }

    if !errors.is_empty() {
        eprintln!("💡 {}", "run `krinjc --help` to view all commands".bold());
    }
}
