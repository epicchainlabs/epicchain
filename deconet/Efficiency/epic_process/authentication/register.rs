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

    /// Gets the email of the user.
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Sets the email of the user.
    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    /// Checks if the password meets the required criteria.
    pub fn is_password_valid(&self) -> bool {
        // Add password validation logic here
        self.password.len() >= 8
    }

    /// Registers the user.
    pub fn register(&self) -> Result<(), String> {
        // Add registration logic here
        if self.is_password_valid() {
            Ok(())
        } else {
            Err("Password is too short".to_string())
        }
    }
}

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

    /// Gets the email of the user.
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Sets the email of the user.
    pub fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    /// Checks if the password meets the required criteria.
    pub fn is_password_valid(&self) -> bool {
        // Add password validation logic here
        self.password.len() >= 8
    }

    /// Registers the user.
    pub fn register(&self) -> Result<(), String> {
        // Add registration logic here
        if self.is_password_valid() {
            Ok(())
        } else {
            Err("Password is too short".to_string())
        }
    }

    /// Updates the username of the user.
    pub fn update_username(&mut self, new_username: &str) {
        self.username = new_username.to_string();
    }

    /// Checks if the user's email is valid.
    pub fn is_email_valid(&self) -> bool {
        // Add email validation logic here
        self.email.contains("@")
    }

    /// Updates the user's password.
    pub fn update_password(&mut self, new_password: &str) {
        self.password = new_password.to_string();
    }

    /// Sends a confirmation email to the user.
    pub fn send_confirmation_email(&self) {
        // Add email sending logic here
        println!("Confirmation email sent to {}", self.email);
    }

    /// Sets the user's email to a new value and sends a confirmation email.
    pub fn set_email_with_confirmation(&mut self, new_email: &str) {
        self.email = new_email.to_string();
        self.send_confirmation_email();
    }

    /// Retrieves a user from the database by username.
    pub fn find_user_by_username(username: &str) -> Option<User> {
        // Add database query logic here
        Some(User::new(username, "user@example.com", "password123"))
    }

    /// Deletes the user's account.
    pub fn delete_account(&self) {
        // Add account deletion logic here
        println!("User account deleted");
    }
}

fn main() {
    let user = User::new("user123", "user@example.com", "password123");

    match user.register() {
        Ok(_) => println!("User registered successfully"),
        Err(err) => println!("Error registering user: {}", err),
    }

    user.send_confirmation_email();
}

fn main() {
    let user = User::new("user123", "user@example.com", "password123");

    match user.register() {
        Ok(_) => println!("User registered successfully"),
        Err(err) => println!("Error registering user: {}", err),
    }
}
