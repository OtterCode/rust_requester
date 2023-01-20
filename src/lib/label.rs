use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    pub message_list_visibility: Option<String>,
    pub label_list_visibility: Option<String>,
    #[serde(rename = "type")]
    pub label_type: String,
    pub messages_total: Option<isize>,
    pub messages_unread: Option<isize>,
    pub threads_total: Option<isize>,
    pub threads_unread: Option<isize>,
    pub color: Option<String>,
}