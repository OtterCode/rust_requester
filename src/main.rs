mod configuration;
mod db;
mod request;
mod process;
mod label;

use configuration::Configuration;

fn main() {

    let db = db::init().unwrap();
    let config = Configuration::init(&db);

    println!("{:?}", config);

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


// Makes an oauth2 request to the Google API based on an API key
// fn requestToken(key: &str) -> String {
//     let client = oauth2::basic::BasicClient::new(
//         oauth2::ClientId::new("".to_string()),
//         Some(oauth2::ClientSecret::new("".to_string())),
//         oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
//         oauth2::TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string()).unwrap(),
//     )
// }