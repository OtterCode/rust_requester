mod config_inputs;
mod label_request_panel;
mod resources;

use config_inputs::ConfigInputs;
use label_request_panel::LabelRequestPanel;
use rust_requester::configuration::{port::Port, Configuration};
use rust_requester::db;

use iced::executor;
use iced::widget::{Column, Text};
use iced::{Application, Command, Element, Settings, Subscription, Theme};

// Iced was chosen here instead of other options because it espouses the
// inimitable Elm Architecture. I'm still sad about how the architect of
// Elm killed his own project. It's a glimpse into what React could have
// been, if only a little more pragmatism had been allowed, and the
// native interfaces hadn't been deprecated. I'm a strong believer in
// functional principles, but I'm not as committed to immutability at all
// costs. This project straddles the line, relying on internal mutation
// that the caller doesn't have need to access whereever possible. 

#[derive(Debug, Clone)]
enum Message {
    ConfigMessage(config_inputs::Message),
    LabelRequestPanelMessage(label_request_panel::Message),
}

pub fn main() -> iced::Result {
    RustRequester::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct RustRequester {
    config: Configuration,
    config_inputs: ConfigInputs,
    db: rusqlite::Connection,
    label_request_panel: LabelRequestPanel,
}

impl Application for RustRequester {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (RustRequester, Command<Self::Message>) {
        let db = db::init().unwrap();
        let config = Configuration::init(&db).unwrap();
        let initial_port = config.local_port.map(Port::as_u16);
        (
            RustRequester {
                config,
                config_inputs: ConfigInputs::new(initial_port),
                db,
                label_request_panel: LabelRequestPanel::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Requester")
    }

    fn update(&mut self, incoming_message: Self::Message) -> Command<Self::Message> {
        match incoming_message {
            Message::ConfigMessage(msg) => {
                self.config = self
                    .config_inputs
                    .update(msg, &mut self.config, &self.db)
                    .unwrap();
                Command::none()
            }
            Message::LabelRequestPanelMessage(msg) => self
                .label_request_panel
                .update(&self.config, &self.db, msg)
                .map(Message::LabelRequestPanelMessage),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("Rust Requester Configuration"))
            .push(
                self.config_inputs
                    .view(&self.config)
                    .map(Message::ConfigMessage),
            )
            .push(
                self.label_request_panel
                    .view()
                    .map(Message::LabelRequestPanelMessage),
            )
            .into()
    }
}
