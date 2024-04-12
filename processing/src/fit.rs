extern crate kdtree;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

use rand::Rng;

use super::TETHER_LENGTH;

use super::preprocess::{Object, Point};

use super::export;

pub struct Parametric {
    pub x_trans: f32,
    pub y_trans: f32,
    pub orders: Vec<Order>,
}

pub struct Order {
    pub direction: i32,
    pub scale: f32,
    pub rotation: f32,
    pub spins: i32,
}

pub fn calculate_best_fit(objects: &Vec<Object>) {
    let mut best_parametric = Vec::<Parametric>::new();

    for object in objects {
        let obs_points: &Vec<Point> = &object.points;

        let kdtree = build_kdtree(obs_points);

        let mut parametric_guesses = Vec::<Parametric>::new();

        seed_parametrics(obs_points, &mut parametric_guesses);

        for parametric in &parametric_guesses {
            let loss = calculate_loss(&kdtree, parametric);
            println!("Loss: {}", loss);
        }

    }
}

fn seed_parametrics(obs_points: &Vec<Point>, parametric_guesses: &mut Vec<Parametric>) {
    // generate parametric guesses based on the observed points
    
    // determine min/max x and y values
    let mut min_x: f32 = std::f32::MAX;
    let mut max_x: f32 = std::f32::MIN;
    let mut min_y: f32 = std::f32::MAX;
    let mut max_y: f32 = std::f32::MIN;

    for point in obs_points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    // generate parametric guesses
    let antispin_r: Parametric = Parametric {
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ((max_x - min_x) / 2.0 + (max_y - min_y) / 2.0) / 4.0,
                rotation: 0.0,
                spins: 2,
            },
            Order {
                direction: -1,
                scale: TETHER_LENGTH,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let inspin_r: Parametric = Parametric {
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ((max_x - min_x) / 2.0 + (max_y - min_y) / 2.0) / 4.0,
                rotation: 0.0,
                spins: 2,
            },
            Order {
                direction: 1,
                scale: TETHER_LENGTH,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let antispin_g: Parametric = Parametric {
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ((max_x - min_x) / 2.0 + (max_y - min_y) / 2.0) / 4.0,
                rotation: std::f32::consts::PI,
                spins: 2,
            },
            Order {
                direction: -1,
                scale: TETHER_LENGTH,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let inspin_g: Parametric = Parametric {
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ((max_x - min_x) / 2.0 + (max_y - min_y) / 2.0) / 4.0,
                rotation: std::f32::consts::PI,
                spins: 2,
            },
            Order {
                direction: 1,
                scale: TETHER_LENGTH,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };




    // let test_points = generate_points(&antispin_g, 100);
    // let _ = export::export_points(&test_points.0, "hand");
    // let _ = export::export_points(&test_points.1, "poi");

    parametric_guesses.push(antispin_r);
    parametric_guesses.push(inspin_r);
    parametric_guesses.push(antispin_g);
    parametric_guesses.push(inspin_g);
}

pub fn calculate_loss(kdtree: &KdTree<f32, usize, [f32; 2]>, parametric: &Parametric) -> f32 {
    let batch_size: i32 = 100;
    let points = generate_points(parametric, batch_size).1;

    let mut loss: f32 = 0.0;
    for point in &points {
        let nearest = kdtree.nearest(&[point.x, point.y], 1, &squared_euclidean).unwrap();
        let nearest_point = nearest[0]; // (distance, point_id)

        loss += nearest_point.0;
    }

    loss / points.len() as f32
}

fn build_kdtree(points: &Vec<Point>) -> KdTree<f32, usize, [f32; 2]> {
    // let _ = export::export_points(&points.1, "test_gen");

    let mut kdtree = KdTree::new(2);
    
    for i in 0..points.len() {
        kdtree.add([points[i].x, points[i].y], i).unwrap();
    }

    kdtree
}

pub fn generate_points(parametric: &Parametric, batch_size: i32) -> (Vec<Point>, Vec<Point>) {

    // generate poi path
    let mut hand_points = Vec::<Point>::new();
    let mut poi_points = Vec::<Point>::new();

    for _ in 0..batch_size {
        let t = rand::thread_rng().gen_range(0.0..2.0*std::f64::consts::PI) as f32;

        let mut point_orders: Vec<Point> = Vec::<Point>::new();

        let initial_point: Point = Point {
            frame_num: -1,
            x: parametric.x_trans,
            y: parametric.y_trans,
            conf: -1.0,
        };
        point_orders.push(initial_point);

        for order in &parametric.orders {
            let prev_point = point_orders.last().unwrap();

            let mut new_point: Point;
            
            // generate point based on direction
            if order.direction == 1 {
                new_point = Point {
                    frame_num: -1,
                    x: prev_point.x + order.scale * (t * (order.spins - 1) as f32 + order.rotation).cos(),
                    y: prev_point.y + order.scale * (t * (order.spins - 1) as f32 + order.rotation).sin(),
                    conf: -1.0,
                };
            } else if order.direction == -1 {
                new_point = Point {
                    frame_num: -1,
                    x: prev_point.x + order.scale * (t * (order.spins - 1) as f32 + order.rotation).cos(),
                    y: prev_point.y - order.scale * (t * (order.spins - 1) as f32 + order.rotation).sin(),
                    conf: -1.0,
                };
            } else {
                panic!("Invalid direction for parametric order");
            }

            point_orders.push(new_point);
        }
        
        hand_points.push(point_orders[1].clone());
        poi_points.push(point_orders.last().unwrap().clone());

    }

    (hand_points, poi_points)

}