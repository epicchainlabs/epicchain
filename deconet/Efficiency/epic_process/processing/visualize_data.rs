use plotters::prelude::*;
use std::collections::HashMap;

/// Struct representing a data point for visualization.
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

/// Struct representing a dataset for visualization.
#[derive(Debug, Clone)]
pub struct DataSet {
    pub label: String,
    pub data: Vec<DataPoint>,
    pub color: RGBColor,
    pub point_style: PointStyle,
}

/// Struct representing a chart for visualization.
#[derive(Debug, Clone)]
pub struct Chart {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub data_sets: Vec<DataSet>,
}

impl Chart {
    /// Function to add a dataset to the chart.
    pub fn add_data_set(&mut self, data_set: DataSet) {
        self.data_sets.push(data_set);
    }

    /// Function to remove a dataset from the chart by label.
    pub fn remove_data_set(&mut self, label: &str) {
        self.data_sets.retain(|data_set| data_set.label != label);
    }

    /// Function to clear all datasets from the chart.
    pub fn clear_data_sets(&mut self) {
        self.data_sets.clear();
    }

    /// Function to plot the chart.
    pub fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("chart.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.title, ("sans-serif", 30.0))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

        chart
            .configure_mesh()
            .x_desc(&self.x_label)
            .y_desc(&self.y_label)
            .draw()?;

        for data_set in &self.data_sets {
            let points: Vec<(f64, f64)> = data_set.data.iter().map(|p| (p.x, p.y)).collect();
            chart
                .draw_series(points.iter().map(|p| Circle::new((*p.0, *p.1), 3, data_set.color.filled())))
                .unwrap()
                .label(&data_set.label)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], data_set.color.filled()));

            chart
                .draw_series(LineSeries::new(
                    points.iter().map(|p| (*p.0, *p.1)),
                    &data_set.color,
                ))?
                .label(&data_set.label);
        }

        chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;

        Ok(())
    }
}

/// Function to visualize data using a chart.
pub fn visualize_data(data: HashMap<String, Vec<(f64, f64)>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut chart = Chart {
        title: "Data Visualization".to_string(),
        x_label: "X Axis".to_string(),
        y_label: "Y Axis".to_string(),
        data_sets: vec![],
    };

    let colors = vec![BLUE, RED, GREEN, YELLOW, PURPLE, CYAN, ORANGE, DARK_GREEN, DARK_BLUE, DARK_RED];

    for (idx, (label, points)) in data.into_iter().enumerate() {
        let color = colors[idx % colors.len()];
        let data_set = DataSet {
            label,
            data: points.iter().map(|(x, y)| DataPoint { x: *x, y: *y }).collect(),
            color,
            point_style: PointStyle::new(),
        };
        chart.add_data_set(data_set);
    }

    chart.plot()?;
    Ok(())
}

fn main() {
    let mut data = HashMap::new();
    data.insert("Dataset 1".to_string(), vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
    data.insert("Dataset 2".to_string(), vec![(0.5, 1.5), (2.5, 3.5), (4.5, 5.5)]);

    visualize_data(data).unwrap();
}
