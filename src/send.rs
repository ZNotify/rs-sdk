use crate::constant::ENDPOINT;
use crate::entity::{Message, MessageOptions, Priority};
use crate::Client;
use std::error::Error;

pub async fn send(
    user_id: String,
    content: String,
    title: Option<String>,
    long: Option<String>,
) -> Result<Message, Box<dyn Error>> {
    Client::create(user_id, Some(String::from(ENDPOINT)))
        .await?
        .send(MessageOptions {
            content,
            title,
            long,
            priority: Some(Priority::Normal),
        })
        .await
}
