use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

/// Struct representing configuration data.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub server_address: String,
    pub port: u16,
    pub max_connections: u32,
}

impl Configuration {
    /// Function to update the server address in the configuration.
    pub fn update_server_address(&mut self, new_address: String) {
        self.server_address = new_address;
    }

    /// Function to update the port in the configuration.
    pub fn update_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    /// Function to update the maximum connections in the configuration.
    pub fn update_max_connections(&mut self, new_max_connections: u32) {
        self.max_connections = new_max_connections;
    }

    /// Function to save the configuration to a JSON file.
    pub fn save_to_file(&self, file_path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Function to load default configuration values.
    pub fn load_default() -> Configuration {
        Configuration {
            server_address: "localhost".to_string(),
            port: 8080,
            max_connections: 100,
        }
    }

    /// Function to merge two configurations, prioritizing the values of `other`.
    pub fn merge(&mut self, other: &Configuration) {
        self.server_address = other.server_address.clone();
        self.port = other.port;
        self.max_connections = other.max_connections;
    }
}

/// Function to read a configuration file and deserialize it into a Configuration struct.
pub fn read_configuration(file_path: &str) -> Result<Configuration, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn main() {
    let file_path = "config.json";
    let default_config = Configuration::load_default();
    let mut config = match read_configuration(file_path) {
        Ok(config) => config,
        Err(_) => {
            println!("Config file not found, using default config.");
            default_config
        }
    };

    println!("Original Configuration: {:?}", config);

    config.update_server_address("new_address".to_string());
    config.update_port(8080);
    config.update_max_connections(100);

    config.merge(&default_config);

    match config.save_to_file(file_path) {
        Ok(_) => println!("Configuration saved successfully"),
        Err(err) => eprintln!("Error saving configuration: {:?}", err),
    }
}
