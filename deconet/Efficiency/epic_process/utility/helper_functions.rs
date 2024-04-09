/// Function to add two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Function to subtract two numbers.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Function to multiply two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Function to divide two numbers.
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

/// Function to check if a number is even.
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

/// Function to check if a number is odd.
pub fn is_odd(num: i32) -> bool {
    num % 2 != 0
}

fn main() {
    // Example usage of helper functions
    println!("Addition: {}", add(5, 3));
    println!("Subtraction: {}", subtract(5, 3));
    println!("Multiplication: {}", multiply(5, 3));
    match divide(6, 3) {
        Ok(result) => println!("Division: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Is 5 even? {}", is_even(5));
    println!("Is 5 odd? {}", is_odd(5));
}
