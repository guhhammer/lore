//! The `lore` binary: everything lives in the library (src/lib.rs).

use std::process::ExitCode;

/// Starts the app. On an error, prints one plain line (not Rust's debug
/// dump) and exits with status 1.
fn main() -> ExitCode {
    match lore_tui::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("lore: {error}");
            ExitCode::FAILURE
        }
    }
}
