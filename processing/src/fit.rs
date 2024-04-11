extern crate kdtree;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

use rand::Rng;
use crate::ARM_RATIO;

use super::preprocess::{Point};

pub struct Parametric {
    pub hand_path: Path,
    pub poi_path: Path,
}

pub struct Path {
    pub direction: i32,
    pub x_trans: f32,
    pub y_trans: f32,
    pub scale: f32,
    pub rotation: f32,
    pub spins: i32,
}

// pub fn calculate_best_fit(objects: Vec<Object>) -> Vec<Parametric> {

// }

pub fn calculate_loss(points: &Vec<Point>, parametric: &Parametric) -> f32 {
    
    let kdtree = build_kdtree(parametric);

    let mut loss: f32 = 0.0;
    for point in points {
        let nearest = kdtree.nearest(&[point.x, point.y], 1, &squared_euclidean).unwrap();
        let nearest_point = nearest[0]; // (distance, point_id)

        loss += nearest_point.0;
    }

    loss / points.len() as f32
}

fn build_kdtree(parametric: &Parametric) -> KdTree<f32, usize, [f32; 2]> {
    let batch_size: i32 = 10000;

    let points: (Vec<Point>, Vec<Point>) = generate_points(parametric, batch_size);

    let mut kdtree = KdTree::new(2);
    
    for i in 0..points.0.len() {
        kdtree.add([points.0[i].x, points.0[i].y], i).unwrap();
    }

    kdtree
}

pub fn generate_points(parametric: &Parametric, batch_size: i32) -> (Vec<Point>, Vec<Point>) {

    // generate poi path
    let mut poi_points = Vec::<Point>::new();
    let mut hand_points = Vec::<Point>::new();

    for _ in 0..batch_size {
        let t = rand::thread_rng().gen_range(0.0..2.0*std::f64::consts::PI) as f32;

        // start generate hand point
        let hand_dir: i32 = parametric.hand_path.direction;
        let hand_x: f32 = parametric.hand_path.x_trans;
        let hand_y: f32 = parametric.hand_path.y_trans;
        let hand_scale: f32 = parametric.hand_path.scale;
        let hand_rot: f32 = parametric.hand_path.rotation;
        let hand_spins: i32 = parametric.hand_path.spins;

        let hand_coordinates = generate_point(t, hand_dir, hand_x, hand_y, hand_scale, hand_rot, hand_spins, true);

        let hand_point = Point {
            frame_num: -1,
            x: hand_coordinates.0,
            y: hand_coordinates.1,
            conf: -1.0,
        };
        
        
        let poi_dir: i32 = parametric.poi_path.direction;
        let poi_x: f32 = parametric.poi_path.x_trans;
        let poi_y: f32 = parametric.poi_path.y_trans;
        let poi_scale: f32 = parametric.poi_path.scale;
        let poi_rot: f32 = parametric.poi_path.rotation;
        let poi_spins: i32 = parametric.poi_path.spins;
        
        
        let poi_coordinates = generate_point(t, poi_dir, poi_x, poi_y, poi_scale, poi_rot, poi_spins, false);
        
        let poi_point = Point {
            frame_num: -1,
            x: poi_coordinates.0 + hand_coordinates.0,
            y: poi_coordinates.1 + hand_coordinates.1,
            conf: -1.0,
        };

        
        hand_points.push(hand_point);
        poi_points.push(poi_point);

    }

    (hand_points, poi_points)

}

pub fn generate_point(t: f32, direction: i32, x: f32, y: f32, scale: f32, rot: f32, spins: i32, is_hand: bool) -> (f32, f32) {
    // TODO: update t to properly represent the range of all possible values
    let mut tether_length = ARM_RATIO;
    if is_hand {
        tether_length = 1.0;
    }

    let coordinates: (f32, f32) = match direction {
        0 => (x, y), // point
        1 => (x + scale * ((t * (spins - 1) as f32 + rot).cos() / tether_length), y - scale * ((t * (spins - 1) as f32 + rot).sin() / tether_length)), // clockwise
        -1 => (x + scale * ((t * (spins - 1) as f32 + rot).cos() / tether_length), y + scale * ((t * (spins - 1) as f32 + rot).sin() / tether_length)), // counter-clockwise
        _ => panic!("Invalid parametric type"),
    };

    coordinates
}