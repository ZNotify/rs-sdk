use crate::entity::{Message, MessageOptions};
use crate::Client;
use std::error::Error;

pub async fn send(
    user_id: String,
    content: String,
    title: Option<String>,
    long: Option<String>,
) -> Result<Message, Box<dyn Error>> {
    Client::create(user_id, None)
        .await?
        .send(MessageOptions {
            content,
            title,
            long,
        })
        .await
}
