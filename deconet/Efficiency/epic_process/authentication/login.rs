use std::collections::HashMap;

/// Struct representing a user.
pub struct User {
    username: String,
    password: String,
    email: String,
    is_admin: bool,
}

impl User {
    /// Creates a new user.
    pub fn new(username: &str, password: &str, email: &str, is_admin: bool) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            is_admin,
        }
    }

    /// Validates the user's credentials.
    pub fn login(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }

    /// Checks if the user is an admin.
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    /// Gets the user's email address.
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Changes the user's password.
    pub fn change_password(&mut self, new_password: &str) {
        self.password = new_password.to_string();
    }

    /// Resets the user's password to a random value.
    pub fn reset_password(&mut self) {
        // Generate a random password (not secure, for demonstration purposes only)
        let new_password = "new_password";
        self.password = new_password.to_string();
    }

    /// Checks if the user's password is expired.
    pub fn is_password_expired(&self) -> bool {
        // For demonstration purposes, assume password expires after 90 days
        // and the user has not changed their password for 90 days
        true
    }

    /// Adds a new user to the system.
    pub fn add_user(username: &str, password: &str, email: &str, is_admin: bool) -> User {
        User::new(username, password, email, is_admin)
    }

    /// Deletes a user from the system.
    pub fn delete_user(user_map: &mut HashMap<String, User>, username: &str) {
        user_map.remove(username);
    }

    /// Retrieves a user from the system by username.
    pub fn get_user(user_map: &HashMap<String, User>, username: &str) -> Option<&User> {
        user_map.get(username)
    }

    /// Updates a user's information.
    pub fn update_user(user_map: &mut HashMap<String, User>, username: &str, new_email: &str, is_admin: bool) {
        if let Some(user) = user_map.get_mut(username) {
            user.email = new_email.to_string();
            user.is_admin = is_admin;
        }
    }
}

fn main() {
    let mut user_map = HashMap::new();
    let user = User::add_user("admin", "password", "admin@example.com", true);
    user_map.insert(user.username.clone(), user);

    // Retrieve user by username
    if let Some(user) = User::get_user(&user_map, "admin") {
        println!("User found: {}", user.username);
    } else {
        println!("User not found");
    }

    // Update user information
    User::update_user(&mut user_map, "admin", "new_admin@example.com", false);

    // Delete user
    User::delete_user(&mut user_map, "admin");
}
