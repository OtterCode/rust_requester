use iced::{Element, widget::{Row, Text, Space, TextInput}, Length};
use rust_requester::configuration::Configuration;
use crate::message::{ ID, Message };

pub struct ConfigInput {
    id: crate::message::ID,
    label: String,
    value: String,
}

impl ConfigInput {
    pub fn new(id: ID, label: String, value: String) -> Self {
        ConfigInput {
            id,
            label,
            value,
        }
    }

    fn update(&self, config: Configuration, db: &rusqlite::Connection) -> Configuration {
        match self.id {
            "api_key" => {
                config.selective_immutable_update(Some(self.value.clone()), None, None, None, None, &db).unwrap()
            }
            _ => {
                panic!("Invalid ID chosen for ConfigInput");
            }
        }

    }

    pub fn render(&self, config: &Configuration) -> Element<Message> {
        Row::new()
            .push(Text::new(self.label.clone()))
            .push(Space::with_width(Length::Units(10)))
            .push(TextInput::new(
                "This is a label too, apparently?",
                &self.value,
                Message::ConfigInputChanged,
            ))
            .into()
    }
}
