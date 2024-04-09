use std::error::Error;
use std::fmt;

/// Custom error type for configuration errors.
#[derive(Debug)]
enum ConfigurationError {
    InvalidValue(String),
    MissingSetting(String),
    FileError(std::io::Error),
    ValidationFailed(String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::InvalidValue(setting) => {
                write!(f, "Invalid value for setting: {}", setting)
            }
            ConfigurationError::MissingSetting(setting) => {
                write!(f, "Missing setting: {}", setting)
            }
            ConfigurationError::FileError(err) => write!(f, "File error: {}", err),
            ConfigurationError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl Error for ConfigurationError {}

/// Function that returns a Result containing a value or a custom error.
fn parse_setting(value: &str) -> Result<i32, ConfigurationError> {
    match value.parse::<i32>() {
        Ok(val) => Ok(val),
        Err(_) => Err(ConfigurationError::InvalidValue(value.to_string())),
    }
}

/// Function that loads a setting from a configuration file.
fn load_setting(file_path: &str, setting_name: &str) -> Result<String, ConfigurationError> {
    let file_content = std::fs::read_to_string(file_path).map_err(ConfigurationError::FileError)?;
    let setting_value = file_content
        .lines()
        .find(|line| line.starts_with(setting_name))
        .ok_or(ConfigurationError::MissingSetting(setting_name.to_string()))?;
    let value = setting_value.split('=').nth(1).unwrap_or("").trim().to_string();
    Ok(value)
}

/// Function that validates a configuration.
fn validate_configuration(config: &HashMap<String, String>) -> Result<(), ConfigurationError> {
    if let Some(value) = config.get("port") {
        if let Err(_) = parse_setting(value) {
            return Err(ConfigurationError::ValidationFailed("Invalid port value".to_string()));
        }
    }
    // Add more validation logic as needed...

    Ok(())
}

/// Function that logs the error message to a file.
fn log_error(error_message: &str) -> Result<(), ConfigurationError> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .map_err(ConfigurationError::FileError)?;

    writeln!(file, "{}", error_message).map_err(ConfigurationError::FileError)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let setting_file = "settings.txt";
    let setting_name = "port";
    let mut config = HashMap::new();
    match load_setting(setting_file, setting_name) {
        Ok(value) => {
            config.insert(setting_name.to_string(), value.clone());
            let port = parse_setting(&value)?;
            println!("Parsed port: {}", port);
        }
        Err(e) => {
            let error_message = format!("Error loading setting: {}", e);
            eprintln!("{}", error_message);
            log_error(&error_message)?;
            return Err(Box::new(e));
        }
    }

    match validate_configuration(&config) {
        Ok(_) => println!("Configuration is valid"),
        Err(e) => {
            let error_message = format!("Configuration validation failed: {}", e);
            eprintln!("{}", error_message);
            log_error(&error_message)?;
            return Err(Box::new(e));
        }
    }

    Ok(())
}
