mod message;
mod config_input;

use rust_requester::configuration::Configuration;
use rust_requester::db;

use iced::executor;
// use iced::widget::canvas::{Cache, Cursor, Geometry, LineCap, Path, Stroke};
// use iced::widget::{canvas, container};
use iced::widget::{ Column, Container, Row, Text };
use iced::{
    Application, Color, Command, Element, Length, Point, Rectangle, Settings,
    Subscription, Theme, Vector,
};

use crate::config_input::ConfigInput;

pub fn main() -> iced::Result {
    RustRequester::run(
        Settings {
            antialiasing: true,
            ..Settings::default()
        }
    )
}

struct RustRequester {
    config: Configuration,
    db: rusqlite::Connection,
}

impl Application for RustRequester {
    type Executor = executor::Default;
    type Message = message::Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (RustRequester, Command<Self::Message>) {
        let db = db::init().unwrap();
        (
            RustRequester {
                config: Configuration::init(&db).unwrap(),
                db,
                text: "Hello, world!".to_string(),
                api_key_input: ConfigInput::new("api_key", "API Key:".to_owned(), "_blank_".to_owned())
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Requester")
    }

    fn update(&mut self, incoming_message: Self::Message) -> Command<Self::Message> {
        match incoming_message {
            message::Message::ConfigInputChanged(value) => {
                self.text = format!("New Value: {}", value);
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new(&self.text))
            .push(self.api_key_input.render(&self.config))
            .into()
    }
}

// fn main() {

//     let db = db::init().unwrap();
//     let config = Configuration::init(&db);

//     let raw_result = request::gmail_label_request(config.unwrap()).unwrap();

//     process::from_json_str(&raw_result, &db);

//     let names: Vec<Result<String, rusqlite::Error>> = db.prepare("SELECT name FROM labels")
//         .expect("Failed to prepare query")
//         .query_map([], |row| {
//             let name: String = row.get(0).unwrap_or("UNNAMED".to_owned());
//             Ok(name)
//         }).expect("Failed to query labels")
//         .collect();

//     for name in names {
//         println!("{}", name.unwrap());
//     }
// }
