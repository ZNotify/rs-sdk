use std::fmt;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Priority {
    #[serde(rename = "high")]
    High,

    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "low")]
    Low,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::High => "high",
            Priority::Normal => "normal",
            Priority::Low => "low",
        }
            .to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Channel {
    #[serde(rename = "WebSocket")]
    WebSocket,

    #[serde(rename = "FCM")]
    FCM,

    #[serde(rename = "WebPush")]
    WebPush,

    #[serde(rename = "WNS")]
    WNS,

    #[serde(rename = "Telegram")]
    Telegram,
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        match self {
            Channel::WebSocket => "WebSocket",
            Channel::FCM => "FCM",
            Channel::WebPush => "WebPush",
            Channel::WNS => "WNS",
            Channel::Telegram => "Telegram",
        }
            .to_string()
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
    pub created_at: String,
    pub priority: Priority,
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
            priority: Some(Priority::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ClientResponse<T> {
    pub(crate) code: i32,
    pub(crate) body: T,
}
