mod preprocess;
use preprocess::{Object, Point};

mod fit;
use fit::{Parametric, Path};

mod export;

pub static TETHER_LENGTH: f32 = 1.2;

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
        x: vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        y: vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    let poi_path = Path {
        x: vec![2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        y: vec![2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    let parametric = Parametric {
        hand_path: hand_path,
        poi_path: poi_path,
    };



    let mut exported_points: (Vec<Point>, Vec<Point>) = fit::generate_points(parametric, 1000);


    

    let _ = export::export_points(exported_points.0);

}