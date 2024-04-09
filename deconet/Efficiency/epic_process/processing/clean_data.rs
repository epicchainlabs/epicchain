use std::collections::HashMap;

/// Function to clean data by removing keys with NaN values.
pub fn clean_data(data: &mut HashMap<String, f64>) {
    data.retain(|_key, value| !value.is_nan());
}

fn main() {
    let mut data = hashmap![
        "Value1".to_string() => 2.0,
        "Value2".to_string() => f64::NAN,
        "Value3".to_string() => 4.0,
    ];

    println!("Original Data: {:?}", data);

    clean_data(&mut data);

    println!("Cleaned Data: {:?}", data);
}
