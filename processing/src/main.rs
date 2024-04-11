mod preprocess;
use preprocess::{Object, Point};

mod fit;
use fit::{Parametric, Path};

mod export;

pub static ARM_RATIO: f32 = 1.2;

fn main() {
    let csv_path: &str = "tracking_data/antispin_tracking.csv";


    // Parse the CSV file
    let points = match preprocess::parse_csv(csv_path) {
        Err(why) => panic!("couldn't parse csv: {}", why),
        Ok(data) => data
    };

    // convert into objects, which contain a vector of points
    let objects: Vec<Object> = preprocess::objectify(points);


    // println!("{:?}", objects);

    let hand_path = Path {
        direction: 1,
        x_trans: 10.0,
        y_trans: 0.0,
        scale: 1.0,
        rotation: 0.0,
        spins: 2,
    };

    let poi_path = Path {
        direction: -1,
        x_trans: 0.0,
        y_trans: 0.0,
        scale: 1.0,
        rotation: 0.0,
        spins: 4,
    };

    let mut parametric = Parametric {
        hand_path: hand_path,
        poi_path: poi_path,
    };



    // let exported_points: (Vec<Point>, Vec<Point>) = fit::generate_points(&parametric, 1000);

    // println!("{:?}", exported_points.1);

    // let _ = export::export_points(&exported_points.0, "hand");
    // let _ = export::export_points(&exported_points.1, "poi");

    // for (idx, object) in objects.iter().enumerate() {
    //     let loss = fit::calculate_loss(&object.points, &parametric);
    //     println!("Loss: {} {} {}", loss, idx, object.points.len());
    // }

    let loss = fit::calculate_loss(&objects[39].points, &parametric);
    println!("Loss: {}", loss);

    let _ = export::export_points(&objects[11].points, "test_obs");
}