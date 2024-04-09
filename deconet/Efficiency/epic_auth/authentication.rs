// authentication.rs

use std::collections::HashMap;

// Define a struct to represent a user
#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// Define a struct to represent an authentication token
#[derive(Debug)]
struct AuthToken {
    token: String,
}

// Define a struct to represent an authentication manager
pub struct AuthManager {
    users: HashMap<String, String>, // HashMap to store username-password pairs
    tokens: HashMap<String, String>, // HashMap to store token-username pairs
}

impl AuthManager {
    // Constructor for AuthManager
    pub fn new() -> Self {
        AuthManager {
            users: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    // Method to register a new user
    pub fn register_user(&mut self, username: &str, password: &str) {
        self.users.insert(username.to_string(), password.to_string());
    }

    // Method to authenticate a user
    pub fn authenticate_user(&mut self, username: &str, password: &str) -> Option<AuthToken> {
        if let Some(expected_password) = self.users.get(username) {
            if *expected_password == password.to_string() {
                let token = format!("{}_token", username); // Generate a simple token
                self.tokens.insert(token.clone(), username.to_string());
                return Some(AuthToken { token });
            }
        }
        None
    }

    // Method to validate an authentication token
    pub fn validate_token(&self, token: &str) -> Option<&str> {
        if let Some(username) = self.tokens.get(token) {
            return Some(username);
        }
        None
    }
}

fn main() {
    let mut auth_manager = AuthManager::new();
    auth_manager.register_user("alice", "password123");

    // Authenticate a user
    if let Some(auth_token) = auth_manager.authenticate_user("alice", "password123") {
        println!("Authentication successful. Token: {:?}", auth_token);

        // Validate the authentication token
        if let Some(username) = auth_manager.validate_token(&auth_token.token) {
            println!("Token is valid for user: {}", username);
        } else {
            println!("Invalid token");
        }
    } else {
        println!("Authentication failed");
    }
}
