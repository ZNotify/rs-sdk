use std::collections::HashMap;
use std::error::Error;

use reqwest::Client as ReqClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constant::ENDPOINT;
use crate::entity::{Channel, ClientResponse, Message, MessageOptions};


pub struct Client {
    endpoint: String,
    user_id: String,
    client: ReqClient,
}

impl Client {
    fn new(user_id: String, endpoint: Option<String>) -> Self {
        Self {
            endpoint: endpoint.unwrap_or_else(|| String::from(ENDPOINT)),
            user_id,
            client: ReqClient::builder()
                .user_agent(format!("znotify-rs-sdk/{}", env!("CARGO_PKG_VERSION")))
                .build()
                .unwrap(),
        }
    }

    pub async fn create(user_secret: String, endpoint: Option<String>) -> Result<Self, Box<dyn Error>> {
        let client = Self::new(user_secret, endpoint);
        let ret = client.check().await;
        if ret.is_ok() {
            Ok(client)
        } else {
            Err(ret.err().unwrap())
        }
    }

    pub async fn check(&self) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/check", self.endpoint))
            .query(&[("user_secret", self.user_id.clone())])
            .send()
            .await?;

        let resp: ClientResponse<bool> = resp.json().await?;
        if !resp.body {
            Err("User secret not valid")?
        }
        Ok(())
    }

    pub async fn send(&self, option: MessageOptions) -> Result<Message, Box<dyn Error>> {
        let content = option.content;
        let title = option.title;
        let long = option.long;
        let priority = option.priority.unwrap_or_default();

        if content.is_empty() {
            Err("Content is required")?
        }

        let mut data = HashMap::new();
        data.insert(
            "title",
            title.unwrap_or_else(|| String::from("Notification")),
        );
        data.insert("content", content);
        data.insert("long", long.unwrap_or_else(String::new));
        data.insert("priority", priority.to_string());

        let resp = self
            .client
            .post(format!("{}/{}/send", self.endpoint, self.user_id))
            .form(&data)
            .send()
            .await?;

        if resp.status().is_success() {
            let resp: ClientResponse<Message> = resp.json().await?;
            Ok(resp.body)
        } else {
            let resp: ClientResponse<String> = resp.json().await?;
            Err(resp.body)?
        }
    }

    pub async fn delete_message(&self, id: String) -> Result<bool, Box<dyn Error>> {
        let client = ReqClient::new();
        let resp = client
            .delete(format!("{}/{}/message/{}", self.endpoint, self.user_id, id))
            .send()
            .await?;

        if resp.status().is_success() {
            let resp: ClientResponse<bool> = resp.json().await?;
            Ok(resp.body)
        } else {
            let resp: ClientResponse<String> = resp.json().await?;
            Err(resp.body)?
        }
    }

    pub async fn create_device(
        &self,
        channel: Channel,
        device_id: String,
        token: Option<String>,
        device_name: Option<String>,
        device_meta: Option<String>,
    ) -> Result<bool, Box<dyn Error>> {
        if !Uuid::parse_str(&device_id).is_ok() {
            Err("Device ID not valid, should be a UUID")?
        }

        let mut data = HashMap::new();
        data.insert("channel", format!("{:?}", channel));
        data.insert("token", token.unwrap_or_default());
        data.insert("device_name", device_name.unwrap_or_default());
        data.insert("device_meta", device_meta.unwrap_or_default());

        let resp = self
            .client
            .put(format!(
                "{}/{}/device/{}",
                self.endpoint, self.user_id, device_id
            ))
            .form(&data)
            .send()
            .await?;

        if resp.status().is_success() {
            let resp: ClientResponse<bool> = resp.json().await?;
            Ok(resp.body)
        } else {
            let resp: ClientResponse<String> = resp.json().await?;
            Err(resp.body)?
        }
    }
}
