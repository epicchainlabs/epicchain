/// Struct representing a user.
pub struct User {
    username: String,
    email: String,
    password: String,
    verified: bool,
}

impl User {
    /// Creates a new user.
    pub fn new(username: &str, email: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            verified: false,
        }
    }

    /// Gets the username of the user.
    pub fn get_username(&self) -> &str {
        &self.username
    }

    /// Gets the verification status of the user.
    pub fn is_verified(&self) -> bool {
        self.verified
    }

    /// Sets the verification status of the user.
    pub fn set_verified(&mut self, verified: bool) {
        self.verified = verified;
    }

    /// Sends a verification email to the user.
    pub fn send_verification_email(&self) {
        // Add email sending logic here
        println!("Verification email sent to {}", self.email);
    }

    /// Verifies the user's account based on a verification token.
    pub fn verify_account(&mut self, token: &str) -> Result<(), String> {
        // Assume the token is valid
        self.set_verified(true);
        Ok(())
    }
}

fn main() {
    let mut user = User::new("user123", "user@example.com", "password123");

    user.send_verification_email();

    // Simulating a verification process
    let token = "valid_verification_token"; // Assume this is a valid verification token
    match user.verify_account(token) {
        Ok(_) => println!("Account verified successfully"),
        Err(err) => println!("Error verifying account: {}", err),
    }

    println!("Is user verified? {}", user.is_verified());
}
