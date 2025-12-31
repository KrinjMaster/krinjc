use colored_text::Colorize;
use krinjc::CError;

pub fn handle_err(err: CError) {
    eprintln!("{} {}\n", "ERROR".red().bold(), err.bold());

    eprintln!("💡 {}", "run `jkrinjc --help` to view all commands".bold());
}
