use cli::error::SuccessOrCargoMakeError;

fn get_name() -> String {
    "makers".to_string()
}

fn main() -> SuccessOrCargoMakeError<std::process::ExitCode> {
    #[cfg(windows)]
    let _ = nu_ansi_term::enable_ansi_support();
    let name = get_name();
    match cli::run_cli(name) {
        Ok(_) => Ok(std::process::ExitCode::SUCCESS).into(),
        Err(e) => SuccessOrCargoMakeError::Err(e),
    }
}
