mod configuration;
mod db;
mod request;
mod process;
mod label;

use configuration::Configuration;

fn main() {

    let db = db::init().unwrap();
    let config = Configuration::init(&db);

    let raw_result = request::gmail_label_request(config.unwrap()).unwrap();

    process::from_json_str(&raw_result, &db);

    let names: Vec<Result<String, rusqlite::Error>> = db.prepare("SELECT name FROM labels")
        .expect("Failed to prepare query")
        .query_map([], |row| {
            let name: String = row.get(0).unwrap_or("UNNAMED".to_owned());
            Ok(name)
        }).expect("Failed to query labels")
        .collect();

    for name in names {
        println!("{}", name.unwrap());
    }
}
