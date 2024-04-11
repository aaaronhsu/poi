use super::preprocess::{Point};

use std::fs::File;
use std::io::prelude::*;

pub fn export_points(points: Vec<Point>, filename: &str) -> std::io::Result<()> {
    let csv_path: String = format!("debug/{}.csv", filename);

    let mut file = File::create(csv_path)?;

    for point in points {
        writeln!(file, "{},{}", point.x, point.y)?;
    }

    Ok(())
}