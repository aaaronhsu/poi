use super::preprocess::{Point};

use std::fs::File;
use std::io::prelude::*;

pub fn export_points(points: &Vec<Point>, filename: &str, norm_factor: Option<(f32, f32)>) -> std::io::Result<()> {
    let csv_path: String = format!("debug/points/{}.csv", filename);

    let mut file = File::create(csv_path)?;

    for point in points {

        match norm_factor {
            Some((x_norm_factor, y_norm_factor)) => {
                writeln!(file, "{},{}", point.x * x_norm_factor, point.y * y_norm_factor);
            },
            None => {
                writeln!(file, "{},{}", point.x, point.y);
            }
        }
    }

    Ok(())
}