mod parse_csv;

fn main() {
    let csv_path: &str = "tracking_data/antispin_tracking.csv";


    // Parse the CSV file
    let points = match parse_csv::parse_csv(csv_path) {
        Err(why) => panic!("couldn't parse csv: {}", why),
        Ok(data) => data
    };

    println!("{:?}", points);
}