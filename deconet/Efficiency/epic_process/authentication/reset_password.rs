/// Struct representing a user.
pub struct User {
    username: String,
    email: String,
    password: String,
}

impl User {
    /// Creates a new user.
    pub fn new(username: &str, email: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    /// Gets the username of the user.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Resets the user's password.
    pub fn reset_password(&mut self, new_password: &str) {
        self.password = new_password.to_string();
    }

    /// Sends a password reset email to the user.
    pub fn send_password_reset_email(&self) {
        // Add email sending logic here
        println!("Password reset email sent to {}", self.email);
    }

    /// Validates the password reset token.
    pub fn validate_reset_token(&self, token: &str) -> bool {
        // Add token validation logic here
        token.len() >= 10
    }

    /// Performs the password reset based on a valid token.
    pub fn reset_password_with_token(&mut self, token: &str, new_password: &str) -> Result<(), String> {
        if self.validate_reset_token(token) {
            self.password = new_password.to_string();
            Ok(())
        } else {
            Err("Invalid reset token".to_string())
        }
    }
}

fn main() {
    let mut user = User::new("user123", "user@example.com", "password123");

    user.send_password_reset_email();

    // Simulating a password reset process
    let token = "valid_reset_token"; // Assume this is a valid reset token
    let new_password = "new_password123";
    match user.reset_password_with_token(token, new_password) {
        Ok(_) => println!("Password reset successfully"),
        Err(err) => println!("Error resetting password: {}", err),
    }
}
