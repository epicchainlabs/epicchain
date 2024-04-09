use std::collections::HashMap;

/// Struct representing a transformation function.
#[derive(Debug)]
pub struct Transformation {
    pub name: String,
    pub description: String,
    pub func: Box<dyn Fn(f64) -> f64>,
}

/// Struct representing a transformation pipeline.
#[derive(Debug)]
pub struct TransformationPipeline {
    pub name: String,
    pub transformations: Vec<Transformation>,
}

impl TransformationPipeline {
    /// Function to add a transformation to the pipeline.
    pub fn add_transformation(&mut self, transformation: Transformation) {
        self.transformations.push(transformation);
    }

    /// Function to remove a transformation from the pipeline by name.
    pub fn remove_transformation(&mut self, name: &str) {
        self.transformations.retain(|transformation| transformation.name != name);
    }

    /// Function to apply the transformation pipeline to a value.
    pub fn apply(&self, value: f64) -> f64 {
        let mut result = value;
        for transformation in &self.transformations {
            result = (transformation.func)(result);
        }
        result
    }

    /// Function to clear all transformations from the pipeline.
    pub fn clear_transformations(&mut self) {
        self.transformations.clear();
    }

    /// Function to get the number of transformations in the pipeline.
    pub fn num_transformations(&self) -> usize {
        self.transformations.len()
    }

    /// Function to get the names of all transformations in the pipeline.
    pub fn transformation_names(&self) -> Vec<String> {
        self.transformations.iter().map(|t| t.name.clone()).collect()
    }
}

/// Function to transform data using a transformation pipeline.
pub fn transform_data(data: HashMap<String, f64>, pipeline: &TransformationPipeline) -> HashMap<String, f64> {
    let mut transformed_data = HashMap::new();
    for (key, value) in data {
        transformed_data.insert(key, pipeline.apply(value));
    }
    transformed_data
}

fn main() {
    let mut pipeline = TransformationPipeline {
        name: "Data Transformation".to_string(),
        transformations: vec![],
    };

    let square_transformation = Transformation {
        name: "Square".to_string(),
        description: "Square the input value".to_string(),
        func: Box::new(|x| x * x),
    };
    pipeline.add_transformation(square_transformation);

    let cube_transformation = Transformation {
        name: "Cube".to_string(),
        description: "Cube the input value".to_string(),
        func: Box::new(|x| x * x * x),
    };
    pipeline.add_transformation(cube_transformation);

    let double_transformation = Transformation {
        name: "Double".to_string(),
        description: "Double the input value".to_string(),
        func: Box::new(|x| x * 2.0),
    };
    pipeline.add_transformation(double_transformation);

    let inverse_transformation = Transformation {
        name: "Inverse".to_string(),
        description: "Compute the inverse of the input value".to_string(),
        func: Box::new(|x| 1.0 / x),
    };
    pipeline.add_transformation(inverse_transformation);

    let data = hashmap![
        "Value1".to_string() => 2.0,
        "Value2".to_string() => 3.0,
        "Value3".to_string() => 4.0,
    ];

    let transformed_data = transform_data(data, &pipeline);

    println!("Transformed Data: {:?}", transformed_data);
    println!("Number of Transformations: {}", pipeline.num_transformations());
    println!("Transformation Names: {:?}", pipeline.transformation_names());
}
