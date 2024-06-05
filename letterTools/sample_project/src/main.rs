use serde_json::Value;
use std::fs;

fn main() {
    let sales_and_products = {
        let file_content =
            fs::read_to_string("./data/sales.json").expect("LogRocket: error reading file");
        serde_json::from_str::<Value>(&file_content).expect("LogRocket: error serializing to JSON")
    };
    println!(
        "{:?}",
        serde_json::to_string_pretty(&sales_and_products)
            .expect("LogRocket: error parsing to JSON")
    );
}