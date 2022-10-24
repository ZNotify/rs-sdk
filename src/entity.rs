use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Message {
    pub id: String,
    pub title: String,
    pub content: String,
    pub long: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct MessageOptions {
    pub content: String,
    pub title: Option<String>,
    pub long: Option<String>,
}

impl Default for MessageOptions {
    fn default() -> Self {
        Self {
            content: String::new(),
            title: None,
            long: None,
        }
    }
}