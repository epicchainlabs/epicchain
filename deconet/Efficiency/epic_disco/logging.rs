use std::fs::OpenOptions;
use std::io::Write;

/// Custom error type for logging errors.
#[derive(Debug)]
pub enum LoggingError {
    FileError(std::io::Error),
}

impl std::fmt::Display for LoggingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggingError::FileError(err) => write!(f, "File error: {}", err),
        }
    }
}

/// Function to log a message to a file.
pub fn log_message(message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "{}", message).map_err(LoggingError::FileError)?;
    Ok(())
}

/// Function to log an error message to a file.
pub fn log_error(error_message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "ERROR: {}", error_message).map_err(LoggingError::FileError)?;
    Ok(())
}

/// Function to log a warning message to a file.
pub fn log_warning(warning_message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "WARNING: {}", warning_message).map_err(LoggingError::FileError)?;
    Ok(())
}

/// Function to log an info message to a file.
pub fn log_info(info_message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "INFO: {}", info_message).map_err(LoggingError::FileError)?;
    Ok(())
}

/// Function to log a debug message to a file.
pub fn log_debug(debug_message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "DEBUG: {}", debug_message).map_err(LoggingError::FileError)?;
    Ok(())
}

/// Function to log a trace message to a file.
pub fn log_trace(trace_message: &str, file_path: &str) -> Result<(), LoggingError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .map_err(LoggingError::FileError)?;

    writeln!(file, "TRACE: {}", trace_message).map_err(LoggingError::FileError)?;
    Ok(())
}
