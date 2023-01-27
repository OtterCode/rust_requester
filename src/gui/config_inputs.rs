use crate::resources;
use iced::{
    widget::{image, Column, Image, Row, Text, TextInput},
    Element,
};

use rust_requester::configuration::Configuration;
use rust_requester::error::Error;

#[derive(Debug, Clone)]
pub enum Message {
    ClientIDChanged(String),
    ClientSecretChanged(String),
    AuthURLChanged(String),
    TokenURLChanged(String),
    LocalPortChanged(String),
}

pub enum ErrorStyle {
    Error,
    Warning,
}

impl ErrorStyle {
    pub fn color(&self) -> iced::Color {
        match self {
            ErrorStyle::Error => iced::Color::from_rgb8(255, 0, 0),
            ErrorStyle::Warning => iced::Color::from_rgb8(200, 200, 0),
        }
    }

    pub fn image(&self) -> Image {
        match self {
            ErrorStyle::Error => image(image::Handle::from_memory(resources::icons::DELETE))
                .width(iced::Length::Units(20))
                .height(iced::Length::Units(20)),
            ErrorStyle::Warning => image(image::Handle::from_memory(resources::icons::WARNING))
                .width(iced::Length::Units(20))
                .height(iced::Length::Units(20)),
        }
    }
}

pub struct ConfigInputs {
    // The port is getting handled differently since it's the only current field with
    // validation. It's better for UI feedback to allow mistakes to be typed out fully.
    pub port_raw_string: String,
    pub port_error_text: Option<String>,
    pub port_error_style: ErrorStyle,
}

impl ConfigInputs {
    fn check_for_low_port(port: u16) -> Option<String> {
        if port < 1024 {
            Some("Ports below 1024 require admin permissions.".to_string())
        } else {
            None
        }
    }

    pub fn new(initial_port: Option<u16>) -> Self {
        let initial_port_value: String = initial_port.map(|p| p.to_string()).unwrap_or_default();
        let port_error_text = initial_port.and_then(Self::check_for_low_port);

        ConfigInputs {
            port_raw_string: initial_port_value,
            port_error_text,
            port_error_style: ErrorStyle::Warning,
        }
    }

    pub fn update(
        &mut self,
        incoming_message: Message,
        config: &mut Configuration,
        db: &rusqlite::Connection,
    ) -> Result<Configuration, Error> {
        match incoming_message {
            Message::ClientIDChanged(value) => {
                config.update_id(db, value.clone())?;
                config.api.id = Some(value);
                Ok(config.clone())
            }
            Message::ClientSecretChanged(value) => {
                config.update_secret(db, value.clone())?;
                config.api.secret = Some(value);
                Ok(config.clone())
            }
            Message::AuthURLChanged(value) => {
                config.update_auth_url(db, value.clone())?;
                config.api.auth_url = Some(value);
                Ok(config.clone())
            }
            Message::TokenURLChanged(value) => {
                config.update_token_url(db, value.clone())?;
                config.api.token_url = Some(value);
                Ok(config.clone())
            }
            Message::LocalPortChanged(value) => {
                let port = value.parse::<u16>();
                match port {
                    Ok(port) => {
                        self.port_error_text = None;
                        self.port_raw_string = value;
                        if let Some(err_text) = Self::check_for_low_port(port) {
                            self.port_error_style = ErrorStyle::Warning;
                            self.port_error_text = Some(err_text);
                        }
                        config.update_local_port(db, port)?;
                        config.local_port = Some(port.into());
                        Ok(config.clone())
                    }
                    Err(err) => {
                        eprintln!("Invalid port number: {}", err);
                        self.port_error_style = ErrorStyle::Error;
                        self.port_error_text = Some("Invalid port number".to_string());
                        self.port_raw_string = value;
                        Ok(config.clone())
                    }
                }
            }
        }
    }

    pub fn view(&self, config: &Configuration) -> Element<Message> {
        let column = Column::new().max_width(500).spacing(10);

        let column = column
            .push(Text::new("Client ID:"))
            .push(TextInput::new(
                "CLIENT ID",
                config.api.id.as_deref().unwrap_or(""),
                Message::ClientIDChanged,
            ))
            .push(Text::new("Client Secret:"))
            .push(
                TextInput::new(
                    "CLIENT SECRET",
                    config.api.secret.as_deref().unwrap_or(""),
                    Message::ClientSecretChanged,
                )
                .password(),
            )
            .push(Text::new("Auth URL:"))
            .push(TextInput::new(
                "AUTH URL",
                config.api.auth_url.as_deref().unwrap_or(""),
                Message::AuthURLChanged,
            ))
            .push(Text::new("Token URL:"))
            .push(TextInput::new(
                "TOKEN URL",
                config.api.token_url.as_deref().unwrap_or(""),
                Message::TokenURLChanged,
            ))
            .push(Text::new("Local Port:"))
            .push(TextInput::new(
                "LOCAL PORT",
                &self.port_raw_string,
                Message::LocalPortChanged,
            ));

        let column = if let Some(error_text) = &self.port_error_text {
            let icon = self.port_error_style.image();
            let color = self.port_error_style.color();

            let row = Row::new()
                .spacing(10)
                .push(icon)
                .push(Text::new(error_text).style(color));

            column.push(row)
        } else {
            column
        };

        column.into()
    }
}
