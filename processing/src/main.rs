mod preprocess;
use preprocess::{Object};

mod fit;

mod export;

pub static TETHER_LENGTH: f32 = 0.8;
pub static DEBUG: bool = true;

fn main() {
    let csv_path: &str = "tracking_data/antispin_tracking.csv";


    // Parse the CSV file
    let points = match preprocess::parse_csv(csv_path) {
        Err(why) => panic!("couldn't parse csv: {}", why),
        Ok(data) => data
    };

    // convert into objects, which contain a vector of points
    let objects: Vec<Object> = preprocess::objectify(points);

    // println!("{:?}", objects[0].points);
    fit::calculate_best_fit(&objects);


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
    //     radius: TETHER_LENGTH,
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