use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug)]
pub enum Priority {
    High,
    Normal,
    Low,
}

impl Priority {
    pub fn to_string(&self) -> String {
        match self {
            Priority::High => "high".to_string(),
            Priority::Normal => "normal".to_string(),
            Priority::Low => "low".to_string(),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

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
    pub priority: Option<Priority>,
}

impl Default for MessageOptions {
    fn default() -> Self {
        Self {
            content: String::new(),
            title: None,
            long: None,
            priority: Some(Priority::Normal),
        }
    }
}
