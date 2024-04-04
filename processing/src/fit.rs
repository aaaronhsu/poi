use rand::Rng;
use crate::TETHER_LENGTH;

use super::preprocess::{Point};

pub struct Parametric {
    pub hand_path: Path,
    pub poi_path: Path,
}

pub struct Path {
    // parametric_type, x, y, scale, rot, spins
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

// pub fn calculate_best_fit(objects: Vec<Object>) -> Vec<Parametric> {

// }

// fn calculate_loss(objects: Vec<Object>, parametric: Parametric) -> f32 {

// }

pub fn generate_points(parametric: Parametric, batch_size: i32) -> (Vec<Point>, Vec<Point>) {

    // generate poi path
    let mut poi_points = Vec::<Point>::new();
    let mut hand_points = Vec::<Point>::new();

    for i in 0..batch_size {
        let mut t = rand::thread_rng().gen_range(0.0..2.0*std::f64::consts::PI) as f32;

        let poi_x_type = parametric.poi_path.x[0] as i32;
        let poi_x_x = parametric.poi_path.x[1];
        let poi_x_y = parametric.poi_path.x[2];
        let poi_x_scale = parametric.poi_path.x[3];
        let poi_x_rot = parametric.poi_path.x[4];
        let poi_x_spins = parametric.poi_path.x[5];
        let poi_x_dir = parametric.poi_path.x[6];

        let poi_y_type = parametric.poi_path.x[0] as i32;
        let poi_y_x = parametric.poi_path.x[1];
        let poi_y_y = parametric.poi_path.x[2];
        let poi_y_scale = parametric.poi_path.x[3];
        let poi_y_rot = parametric.poi_path.x[4];
        let poi_y_spins = parametric.poi_path.x[5];
        let poi_y_dir = parametric.poi_path.x[6];

        let poi_x = generate_point(t, poi_x_type, poi_x_x, poi_x_y, poi_x_scale, poi_x_rot, poi_x_spins);
        let poi_y = generate_point(t, poi_y_type, poi_y_x, poi_y_y, poi_y_scale, poi_y_rot, poi_y_spins);

        let poi_point = Point {
            frame_num: -1,
            x: poi_x.0,
            y: poi_y.1,
            conf: -1.0,
        };
        poi_points.push(poi_point); 

        let hand_x_type = parametric.hand_path.x[0] as i32;
        let hand_x_x = parametric.hand_path.x[1];
        let hand_x_y = parametric.hand_path.x[2];
        let hand_x_scale = parametric.hand_path.x[3];
        let hand_x_rot = parametric.hand_path.x[4];
        let hand_x_spins = parametric.hand_path.x[5];
        let hand_x_dir = parametric.hand_path.x[6];

        let hand_y_type = parametric.hand_path.y[0] as i32;
        let hand_y_x = parametric.hand_path.y[1];
        let hand_y_y = parametric.hand_path.y[2];
        let hand_y_scale = parametric.hand_path.y[3];
        let hand_y_rot = parametric.hand_path.y[4];
        let hand_y_spins = parametric.hand_path.y[5];
        let hand_y_dir = parametric.hand_path.y[6];

        let hand_x = generate_point(t, hand_x_type, hand_x_x, hand_x_y, hand_x_scale, hand_x_rot, hand_x_spins);
        let hand_y = generate_point(t, hand_y_type, hand_y_x, hand_y_y, hand_y_scale, hand_y_rot, hand_y_spins);

        let hand_point = Point {
            frame_num: -1,
            x: hand_x.0,
            y: hand_y.1,
            conf: -1.0,
        };
        hand_points.push(hand_point);
    }

    (poi_points, hand_points)

}

pub fn generate_point(t: f32, parametric_type: i32, x: f32, y: f32, scale: f32, rot: f32, spins: f32) -> (f32, f32) {
    // TODO: update t to properly represent the range of all possible values

    let coordinates: (f32, f32) = match parametric_type {
        0 => (x, y), // point
        1 => (x + scale * t.cos(), y + scale * t.sin()), // circle
        2 => (x + scale * ((t * (spins - 1.0) + rot).cos() / TETHER_LENGTH), y - scale * ((t * (spins - 1.0) + rot).sin() / TETHER_LENGTH)), // antispin
        3 => (x + scale * ((t * (spins - 1.0) + rot).cos() / TETHER_LENGTH), y + scale * ((t * (spins - 1.0) + rot).sin() / TETHER_LENGTH)), // inspin
        _ => panic!("Invalid parametric type"),
    };

    coordinates
}