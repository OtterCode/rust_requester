pub type ID = &'static str;

#[derive(Debug, Clone)]
pub enum Message {
    ConfigInputChanged(String),
}
