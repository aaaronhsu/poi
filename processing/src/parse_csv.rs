use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_csv(file_path: &str) -> io::Result<Vec<Vec<f32>>> {
    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let cells: Vec<f32> = line.split(',')
                                .filter_map(|s| s.parse::<f32>().ok())
                                .collect();
        data.push(cells);
    }

    Ok(data)
}