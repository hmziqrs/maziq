mod catalog;
mod cli;
mod templates;

fn main() -> std::process::ExitCode {
    match cli::run() {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            std::process::ExitCode::FAILURE
        }
    }
}
