use crate::entity::{Message, MessageOptions};
use crate::Client;
use std::error::Error;

pub async fn send(
    content: String,
    title: Option<String>,
    long: Option<String>,
) -> Result<Message, Box<dyn Error>> {
    let client = Client::create("user_id".to_string(), None).await.unwrap();
    client
        .send(MessageOptions {
            content,
            title,
            long,
        })
        .await
}
