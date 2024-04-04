mod preprocess;

fn main() {
    let csv_path: &str = "tracking_data/antispin_tracking.csv";


    // Parse the CSV file
    let points = match preprocess::parse_csv(csv_path) {
        Err(why) => panic!("couldn't parse csv: {}", why),
        Ok(data) => data
    };

    // convert into objects, which contain a vector of points
    let objects: Vec<Object> = preprocess::objectify(points);

    println!("{:?}", objects);
}