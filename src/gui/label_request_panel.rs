use iced::{
    widget::{Button, Column, Text},
    Command, Element,
};
use rust_requester::{configuration::Configuration, error::Error, request::GmailLabelRequest};
use webbrowser;

#[derive(Debug, Clone)]
pub enum Message {
    MakeLabelRequest,
    MakeOauthRequest,
    Cancel,
    ReceivedOauthTargetURL(Result<GmailLabelRequest, String>),
    ReceivedOauthCode(Result<String, Error>),
    ReceivedOauthToken(Result<GmailLabelRequest, Error>),
    LabelsReceived(Result<(String, GmailLabelRequest), Error>),
}

pub struct LabelRequestPanel {
    request: Option<GmailLabelRequest>,
    display_errors: Option<String>,
    labels: Option<Vec<String>>,
}

impl LabelRequestPanel {
    pub fn new() -> Self {
        LabelRequestPanel {
            request: None,
            display_errors: None,
            labels: None,
        }
    }

    pub fn update(
        &mut self,
        config: &Configuration,
        db: &rusqlite::Connection,
        incoming_message: Message,
    ) -> Command<Message> {
        self.display_errors = None;
        match incoming_message {
            Message::MakeLabelRequest => {
                let request = self.request.clone();
                match request {
                    Some(request) => {
                        self.labels = None;
                        let cloned_request = request.clone();
                        self.request = Some(request);
                        return Command::perform(
                            cloned_request.get_labels(),
                            Message::LabelsReceived,
                        );
                    }
                    None => {
                        self.display_errors =
                            Some("App not authorized. Please cancel and try again.".to_string());
                    }
                }
            }
            Message::MakeOauthRequest => {
                let config = config.clone();
                return Command::perform(
                    async move {
                        GmailLabelRequest::new(&config).await.map_err(|err| {
                            eprintln!("{}", err);
                            err.to_string()
                        })
                    },
                    Message::ReceivedOauthTargetURL,
                );
            }
            Message::ReceivedOauthTargetURL(res) => {
                let config = config.clone();
                match res {
                    Ok(request) => {
                        let try_browser = webbrowser::open(request.oauth_auth_url.as_str());
                        if let Err(err) = try_browser {
                            eprintln!("{}", err);
                            self.display_errors =
                                Some("Could not open default browser.".to_owned());
                        }
                        let (request, server) = request.oauth2_token_receiver(config);
                        self.request = Some(request);
                        return Command::perform(async move {
                                match server.await {
                                    Ok(result) => result,
                                    Err(err) => Err(Error::from(Box::from(err)))
                                }
                            },
                            Message::ReceivedOauthCode,
                        );
                    }
                    Err(err) => {
                        self.display_errors = Some(err);
                    }
                }
            },
            Message::ReceivedOauthCode(maybe_code) => {
                match maybe_code {
                    Ok(code) => {
                        let maybe_request = self.request.clone();
                        match maybe_request {
                            Some(request) => {
                                return Command::perform(
                                    request.oauth2_token_verification(code),
                                    Message::ReceivedOauthToken
                                );                                
                            },
                            None => {
                                self.display_errors = Some("Request canceled, please retry.".to_string());
                            }
                        }
                    },
                    Err(err) => {
                        self.display_errors = Some(err.to_string());
                    }
                }
            },
            Message::ReceivedOauthToken(request) => match request {
                Ok(request) => {
                    let cloned_request = request.clone();
                    self.request = Some(request);
                    return Command::perform(cloned_request.get_labels(), Message::LabelsReceived);
                }
                Err(err) => {
                    self.display_errors = Some(err.to_string());
                }
            },
            Message::LabelsReceived(res) => match res {
                Ok((json, request)) => {
                    self.request = Some(request);
                    rust_requester::process::from_json_str(&json, db);
                    self.labels = Some(
                        rust_requester::db::get_labels(&db)
                            .iter()
                            .filter_map(|item| item.as_ref().ok())
                            .map(String::to_owned)
                            .collect(),
                    );
                }
                Err(err) => {
                    self.display_errors = Some(err.to_string());
                }
            },
            Message::Cancel => {
                self.display_errors = None;
                let mut request = self.request.take();
                request.as_mut().map(GmailLabelRequest::kill);
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let column = Column::new();

        let request_button = if self.display_errors.is_none() {
            match self.request.as_ref() {
                Some(req) => {
                    if req.token.is_some() {
                        Button::new(Text::new("Get/Refresh Labels"))
                            .on_press(Message::MakeLabelRequest)
                    } else {
                        Button::new(Text::new("Cancel (awaiting browser authorization)"))
                            .on_press(Message::Cancel)
                    }
                }
                None => Button::new(Text::new("Authorize App")).on_press(Message::MakeOauthRequest),
            }
        } else {
            Button::new(Text::new("Reset Request")).on_press(Message::Cancel)
        };

        let column = column.push(request_button);

        let column = if let Some(err) = self.display_errors.clone() {
            column.push(Text::new(err))
        } else {
            column
        };

        let column = if let Some(labels) = &self.labels {
            labels
                .iter()
                .fold(column, |acc, label| acc.push(Text::new(label)))
        } else {
            column
        };

        column.into()
    }
}
