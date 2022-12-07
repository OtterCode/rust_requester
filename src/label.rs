use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    pub message_list_visibility: String,
    pub label_list_visibility: String,
    #[serde(rename = "type")]
    pub label_type: String,
    pub messages_total: isize,
    pub messages_unread: isize,
    pub threads_total: isize,
    pub threads_unread: isize,
    pub color: String,
}