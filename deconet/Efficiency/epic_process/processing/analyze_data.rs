use std::collections::HashMap;

/// Function to analyze data by computing the sum of the values.
pub fn sum(data: &HashMap<String, f64>) -> f64 {
    data.values().sum()
}

/// Function to analyze data by computing the mean of the values.
pub fn mean(data: &HashMap<String, f64>) -> f64 {
    let sum: f64 = data.values().sum();
    let count = data.len() as f64;
    sum / count
}

/// Function to analyze data by computing the median of the values.
pub fn median(data: &mut HashMap<String, f64>) -> f64 {
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

/// Function to analyze data by computing the standard deviation of the values.
pub fn standard_deviation(data: &HashMap<String, f64>) -> f64 {
    let mean_value = mean(data);
    let variance = data.values().map(|v| (v - mean_value).powi(2)).sum::<f64>() / (data.len() as f64);
    variance.sqrt()
}

fn main() {
    let data = hashmap![
        "Value1".to_string() => 2.0,
        "Value2".to_string() => 3.0,
        "Value3".to_string() => 4.0,
    ];

    println!("Original Data: {:?}", data);

    let sum_value = sum(&data);
    println!("Sum: {}", sum_value);

    let mean_value = mean(&data);
    println!("Mean: {}", mean_value);

    let mut cloned_data = data.clone();
    let median_value = median(&mut cloned_data);
    println!("Median: {}", median_value);

    let standard_deviation_value = standard_deviation(&data);
    println!("Standard Deviation: {}", standard_deviation_value);
}
