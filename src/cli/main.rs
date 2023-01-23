use clap::Parser;

use rust_requester::request;
use rust_requester::process;

use rust_requester::db;
use rust_requester::configuration::{ Configuration, port::Port };

#[derive(Parser, Debug)]
#[command(about = include_str!("./README.md"), long_about = None)]
struct Args {
    /// Reset the API configuration to blank.
    #[arg(short, long)]
    reset: bool
}

fn main() {

    let args = Args::parse();

    let db = db::init().unwrap();

    let mut config: Configuration = if args.reset {
        Configuration::reset(&db).expect("Failed to reset configuration.")
    } else {
        Configuration::init(&db).expect("Failed to initialize configuration.")
    };

    if !config.is_complete() { 
        config = fill_config(config, &db).expect("Could not save new configuration.");
    }

    let url_callback = |url: &str| {
        println!("Please visit the following URL and follow the instructions to authorize this application:");
        println!("{}", url);
    };

    let raw_result = request::gmail_label_request(config, &url_callback).unwrap();

    process::from_json_str(&raw_result, &db);

    let names: Vec<Result<String, rusqlite::Error>> = db.prepare("SELECT name FROM labels")
        .expect("Failed to prepare query.")
        .query_map([], |row| {
            let name: String = row.get(0).unwrap_or("UNNAMED".to_owned());
            Ok(name)
        }).expect("Failed to query labels.")
        .collect();

    for name in names {
        println!("{}", name.unwrap());
    }
}

fn has_length(s: &String) -> bool {
    s.len() > 0
}

fn fill_config(mut configuration: Configuration, db: &db::Connection) -> Result<Configuration, Box<dyn std::error::Error>> {
    println!("Please enter missing API credentials");
    let mut rl = rustyline::Editor::<()>::new().unwrap();

    let api_id = if configuration.api.id.is_some() {
        configuration.api.id.clone()
    } else {
        rl.readline("API ID: ").ok()
    }.filter(has_length);

    let api_secret = if configuration.api.secret.is_some() {
        configuration.api.secret.clone()
    } else {
        rl.readline("API Secret: ").ok()
    }.filter(has_length);

    let auth_url = if configuration.api.auth_url.is_some() {
        configuration.api.auth_url.clone()
    } else {
        rl.readline("Auth URL: ").ok()
    }.filter(has_length);

    let token_url = if configuration.api.token_url.is_some() {
        configuration.api.token_url.clone()
    } else {
        rl.readline("Token URL: ").ok()
    }.filter(has_length);

    let local_port = if configuration.local_port.is_some() {
        configuration.local_port
    } else {
        rl.readline("Local Port: ").ok()
            .and_then(|s| s.parse::<u16>().map(Port::from).ok())
    };


    configuration.update_config(api_id, api_secret, auth_url, token_url, local_port, &db)

}