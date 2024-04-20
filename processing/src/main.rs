mod preprocess;
use preprocess::{Object, Point};

mod fit;
use fit::{Parametric};

mod export;

pub static TETHER_RATIO: f32 = 0.8;
pub static DEBUG: bool = true;
pub static EXPORT_STEPS: bool = true;

fn main() {
    let csv_path: &str = "tracking_data/sim_antispin.csv";


    // Parse the CSV file
    let points = match preprocess::parse_csv(csv_path) {
        Err(why) => panic!("couldn't parse csv: {}", why),
        Ok(data) => data
    };

    // convert into objects, which contain a vector of points
    let object_data: (Vec<Object>, f32, f32) = preprocess::objectify(&points);

    let objects = object_data.0;
    let x_norm_factor = object_data.1;
    let y_norm_factor = object_data.2;

    // println!("{:?}", objects[0].points);
    let best_parametric: Parametric = fit::calculate_best_fit(&objects, x_norm_factor, y_norm_factor);


    println!("Best parametric: {:?}", best_parametric.name);
    let mut test_points: (Vec<Point>, Vec<Point>) = fit::generate_points(&best_parametric, 10000);

    let _ = export::export_points(&test_points.0, "best_hand", Some((x_norm_factor, y_norm_factor)));
    let _ = export::export_points(&test_points.1, "best_poi", Some((x_norm_factor, y_norm_factor)));

    println!("{}, {}", x_norm_factor, y_norm_factor);


    // println!("{:?}", objects);

    // let hand_path = Path {
    //     direction: 1,
    //     x_trans: 10.0,
    //     y_trans: 0.0,
    //     radius: 1.0,
    //     rotation: 0.0,
    //     spins: 2,
    // };

    // let poi_path = Path {
    //     direction: -1,
    //     x_trans: 0.0,
    //     y_trans: 0.0,
    //     radius: TETHER_RATIO,
    //     rotation: 0.0,
    //     spins: 4,
    // };

    // let mut parametric = Parametric {
    //     hand_path: hand_path,
    //     poi_path: poi_path,
    // };



    // let exported_points: (Vec<Point>, Vec<Point>) = fit::generate_points(&parametric, 1000);

    // println!("{:?}", exported_points.1);


    // for (idx, object) in objects.iter().enumerate() {
    //     let loss = fit::calculate_loss(&object.points, &parametric);
    //     println!("Loss: {} {} {}", loss, idx, object.points.len());
    // }

    // let loss = fit::calculate_loss(&objects[0].points, &parametric);
    // println!("Loss: {}", loss);

    // let _ = export::export_points(&objects[0].points, "antispin");
}