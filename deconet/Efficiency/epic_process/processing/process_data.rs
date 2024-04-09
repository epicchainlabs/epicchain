use std::collections::HashMap;

/// Function to process data by applying a specified operation to each value.
pub fn process_data(data: HashMap<String, f64>, operation: &dyn Fn(f64) -> f64) -> HashMap<String, f64> {
    let mut processed_data = HashMap::new();
    for (key, value) in data {
        let processed_value = operation(value);
        processed_data.insert(key, processed_value);
    }
    processed_data
}

/// Function to calculate the mean of the values in the data.
pub fn mean(data: HashMap<String, f64>) -> f64 {
    let sum: f64 = data.values().sum();
    let count = data.len() as f64;
    sum / count
}

/// Function to calculate the median of the values in the data.
pub fn median(data: HashMap<String, f64>) -> f64 {
    let mut sorted_values: Vec<_> = data.values().cloned().collect();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count = sorted_values.len();
    if count % 2 == 0 {
        let mid = count / 2;
        (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
    } else {
        sorted_values[count / 2]
    }
}

/// Function to calculate the standard deviation of the values in the data.
pub fn standard_deviation(data: HashMap<String, f64>) -> f64 {
    let mean_value = mean(data.clone());
    let variance = data.values().map(|v| (v - mean_value).powi(2)).sum::<f64>() / (data.len() as f64);
    variance.sqrt()
}

fn main() {
    let data = hashmap![
        "Value1".to_string() => 2.0,
        "Value2".to_string() => 3.0,
        "Value3".to_string() => 4.0,
    ];

    let squared_data = process_data(data.clone(), &|x| x * x);
    println!("Squared Data: {:?}", squared_data);

    let cubed_data = process_data(data.clone(), &|x| x * x * x);
    println!("Cubed Data: {:?}", cubed_data);

    let doubled_data = process_data(data.clone(), &|x| x * 2.0);
    println!("Doubled Data: {:?}", doubled_data);

    let mean_value = mean(data.clone());
    println!("Mean: {}", mean_value);

    let median_value = median(data.clone());
    println!("Median: {}", median_value);

    let standard_deviation_value = standard_deviation(data.clone());
    println!("Standard Deviation: {}", standard_deviation_value);
}
