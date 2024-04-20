extern crate kdtree;

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

use rand::Rng;

use super::{TETHER_RATIO, DEBUG, EXPORT_STEPS};

use super::preprocess::{Object, Point};

use super::export;

#[derive(Clone)]
pub struct Parametric {
    pub name: String,
    pub x_trans: f32,
    pub y_trans: f32,
    pub orders: Vec<Order>,
}

#[derive(Clone)]
pub struct Order {
    pub direction: i32,
    pub scale: f32,
    pub rotation: f32,
    pub spins: i32,
}

pub fn calculate_best_fit(objects: &Vec<Object>, x_norm_factor: f32, y_norm_factor: f32) -> Parametric {
    let mut best_parametric_overall: Parametric = Parametric {
        name: "none".to_string(),
        x_trans: 0.0,
        y_trans: 0.0,
        orders: vec![]};
    let mut best_loss_overall = std::f32::MAX;

    for object in objects {
        let obs_points: &Vec<Point> = &object.points;

        let mut parametric_guesses = Vec::<Parametric>::new();

        seed_parametrics(obs_points, &mut parametric_guesses);

        let learning_rate: f32 = 5.0;


        for parametric in &mut parametric_guesses {

            let mut losses: Vec<f32> = Vec::<f32>::new();

            let mut best_parametric: Parametric = parametric.clone(); // this is the current best parametric
            let mut best_loss: f32 = std::f32::MAX;

            for step_num in 0..50 {
                let pre_kdtree = build_kdtree(&parametric);
                let pre_loss = calculate_loss(&pre_kdtree, obs_points);
                losses.push(pre_loss);

                if pre_loss < best_loss {
                    best_loss = pre_loss;
                    best_parametric = parametric.clone();
                }

                // if last 5 losses are within 5% of one another, terminate because convergence has been reached
                if losses.len() >= 5 {
                    let last_five_losses = &losses[(losses.len() - 5)..];
                    let max_loss = *last_five_losses.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
                    let min_loss = *last_five_losses.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
            
                    if (max_loss - min_loss) / min_loss <= 0.05 { // change is within 5%
                        if DEBUG {
                            println!("Convergence reached");
                        }
                        break;
                    }
                }
    
                let mut steps: Vec::<f32> = vec![0.0, 0.0, 0.0];
    
                // x_trans
                {
                    parametric.x_trans += 0.03;
    
                    let x_kdtree = build_kdtree(&parametric);
                    let x_loss = calculate_loss(&x_kdtree, obs_points);
                    steps[0] = parametric.x_trans * (pre_loss - x_loss);

                    println!("pre:{} post:{} step:{}", pre_loss, x_loss, steps[0]);
    
                    parametric.x_trans -= 0.03;
                }
    
                // y_trans
                {
                    parametric.y_trans += 0.03;
                    let y_kdtree = build_kdtree(&parametric);
                    let y_loss = calculate_loss(&y_kdtree, obs_points);
                    steps[1] = parametric.y_trans * (pre_loss - y_loss);
    
                    parametric.y_trans -= 0.03;
                }
    
                // scale
                // {
                //     parametric.orders[0].scale += 0.1;
                    
                //     let scale_kdtree = build_kdtree(&parametric);
                //     let scale_loss = calculate_loss(&scale_kdtree, obs_points);
                //     steps[2] = parametric.orders[0].scale * (pre_loss - scale_loss);
    
                //     parametric.orders[0].scale -= 0.1;
                // }
    
                parametric.x_trans += steps[0] * learning_rate;
                parametric.y_trans += steps[1] * learning_rate;
                parametric.orders[0].scale += steps[2] * learning_rate;

                if EXPORT_STEPS {
                    let test_points = generate_points(&parametric, 1000);
                    let _ = export::export_points(&test_points.0, &format!("steps/hand{}", step_num), Some((x_norm_factor, y_norm_factor)));
                    let _ = export::export_points(&test_points.1, &format!("steps/poi{}", step_num), Some((x_norm_factor, y_norm_factor)));
                }
            }


            parametric.x_trans = best_parametric.x_trans;
            parametric.y_trans = best_parametric.y_trans;
            parametric.orders[0].scale = best_parametric.orders[0].scale;

            if best_loss < best_loss_overall {
                best_loss_overall = best_loss;
                best_parametric_overall = best_parametric.clone();
            }



            println!("{} Loss: {}", parametric.name, best_loss);

            if DEBUG {
                let test_points = generate_points(&parametric, 10000);
                let _ = export::export_points(&test_points.0, &format!("{}_hand", &parametric.name), Some((x_norm_factor, y_norm_factor)));
                let _ = export::export_points(&test_points.1, &format!("{}_poi", &parametric.name), Some((x_norm_factor, y_norm_factor)));
            }
        }
    }

    best_parametric_overall

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

    println!("min_x: {} max_x: {} min_y: {} max_y: {}", min_x, max_x, min_y, max_y);

    let ARM_LENGTH: f32 = ((max_x - min_x) / 2.0 + (max_y - min_y) / 2.0) / 4.0;

    // generate parametric guesses
    let antispin_r: Parametric = Parametric {
        name: "antispin_r".to_string(),
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        // x_trans: 0.0,
        // y_trans: 0.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ARM_LENGTH,
                rotation: 0.0,
                spins: 2,
            },
            Order {
                direction: -1,
                scale: ARM_LENGTH * TETHER_RATIO,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let inspin_r: Parametric = Parametric {
        name: "inspin_r".to_string(),
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        // x_trans: 0.0,
        // y_trans: 0.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ARM_LENGTH,
                rotation: 0.0,
                spins: 2,
            },
            Order {
                direction: 1,
                scale: ARM_LENGTH * TETHER_RATIO,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let antispin_g: Parametric = Parametric {
        name: "antispin_g".to_string(),
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        // x_trans: 0.0,
        // y_trans: 0.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ARM_LENGTH,
                rotation: std::f32::consts::PI,
                spins: 2,
            },
            Order {
                direction: -1,
                scale: ARM_LENGTH * TETHER_RATIO,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    let inspin_g: Parametric = Parametric {
        name: "inspin_g".to_string(),
        x_trans: (max_x + min_x) / 2.0,
        y_trans: (max_y + min_y) / 2.0,
        // x_trans: 0.0,
        // y_trans: 0.0,
        orders: vec![
            Order {
                direction: 1,
                scale: ARM_LENGTH,
                rotation: std::f32::consts::PI,
                spins: 2,
            },
            Order {
                direction: 1,
                scale: ARM_LENGTH * TETHER_RATIO,
                rotation: 0.0,
                spins: 4,
            },
        ],
    };

    parametric_guesses.push(antispin_r);
    // parametric_guesses.push(inspin_r);
    // parametric_guesses.push(antispin_g);
    // parametric_guesses.push(inspin_g);
}

pub fn calculate_loss(kdtree: &KdTree<f32, usize, [f32; 2]>, observed_points: &Vec<Point>) -> f32 {

    let mut loss: f32 = 0.0;
    for point in observed_points {
        let nearest = kdtree.nearest(&[point.x, point.y], 1, &squared_euclidean).unwrap();
        let nearest_point = nearest[0]; // (distance, point_id)

        loss += nearest_point.0 * nearest_point.0;
    }

    loss / observed_points.len() as f32
}

fn build_kdtree(parametric: &Parametric) -> KdTree<f32, usize, [f32; 2]> {
    // let _ = export::export_points(&points.1, "test_gen");
    let batch_size: i32 = 50;
    let points = generate_points(parametric, batch_size).1;

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
            let prev_point: &Point = point_orders.last().unwrap();

            let new_point: Point;
            
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
        poi_points.push(point_orders[2].clone());

    }

    (hand_points, poi_points)

}