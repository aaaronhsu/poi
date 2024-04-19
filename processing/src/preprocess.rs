use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Object {
    pub obj_id: i32, // primary key
    pub obj_type: i32, // 0 = poi, 1 = hand
    pub points: Vec<Point>, // should be sorted based on insertion order
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Point {
    pub frame_num: i32,
    pub x: f32,
    pub y: f32,
    // pub x_norm_factor: f32,
    // pub y_norm_factor: f32,
    pub conf: f32,
}

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
        if cells.len() == 0 {
            continue;
        }
        data.push(cells);
    }

    Ok(data)
}

pub fn objectify(data: &Vec<Vec<f32>>) -> (Vec<Object>, f32, f32) {
    let mut id_to_type = HashMap::<i32, i32>::new();
    let mut id_to_points = HashMap::<i32, Vec<Point>>::new();

    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;

    for row in data {
        let x = row[3];
        let y = row[4];

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }
    }

    
    for row in data {
        let obj_id = row[0] as i32;
        let obj_type = row[1] as i32;
        let frame_num = row[2] as i32;
        let x = row[3] as f32 / max_x;
        let y = row[4] as f32 / max_y;
        let conf = row[5];

        let point = Point {
            frame_num: frame_num,
            x: x,
            y: y,
            // x_norm_factor: max_x,
            // y_norm_factor: max_y,
            conf: conf,
        };

        if !id_to_type.contains_key(&obj_id) {
            id_to_type.insert(obj_id, obj_type);
        }

        if !id_to_points.contains_key(&obj_id) {
            id_to_points.insert(obj_id, Vec::new());
        }

        let points = id_to_points.get_mut(&obj_id).unwrap();
        points.push(point);
    }

    let mut objects = Vec::<Object>::new();
    for (obj_id, obj_type) in id_to_type.iter() {
        let points = id_to_points.get(obj_id).unwrap();

        let object = Object {
            obj_id: *obj_id,
            obj_type: *obj_type,
            points: points.clone(),
        };
        objects.push(object);
    }

    (objects, max_x, max_y)
}