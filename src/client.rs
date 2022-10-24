use std::collections::HashMap;
use std::error::Error;

use reqwest::Client as ReqClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constant::ENDPOINT;
use crate::entity::{Message, MessageOptions};

#[derive(Debug, Serialize, Deserialize)]
struct ClientResponse<T> {
    code: i32,
    body: T,
}

#[derive(Debug)]
pub enum ChannelType {
    WebSocket,
    FCM
}

impl ChannelType {
    fn as_str(&self) -> &'static str {
        match self {
            ChannelType::WebSocket => "websocket",
            ChannelType::FCM => "fcm"
        }
    }
}

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
            client: ReqClient::new(),
        }
    }

    pub async fn create(user_id: String, endpoint: Option<String>) -> Result<Self, Box<dyn Error>> {
        let client = Self::new(user_id, endpoint);
        let ret = client.check().await;
        if ret.is_ok(){
            Ok(client)
        } else {
            Err(ret.err().unwrap())
        }
    }

    pub async fn check(&self) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .get(format!("{}/check", self.endpoint))
            .query(&[("user_id", self.user_id.clone())])
            .send()
            .await?;

        let resp: ClientResponse<bool> = resp.json().await?;
        if !resp.body {
            Err("User ID not valid")?
        }
        Ok(())
    }

    pub async fn send(&self, option: MessageOptions) -> Result<Message, Box<dyn Error>> {
        let content = option.content;
        let title = option.title;
        let long = option.long;

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

    pub async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let client = ReqClient::new();
        client
            .delete(format!("{}/{}/{}", self.endpoint, self.user_id, id))
            .send()
            .await?;
        Ok(())
    }

    pub async fn register(
        &self,
        channel: ChannelType,
        token: String,
        device_id: String,
    ) -> Result<(), Box<dyn Error>> {
        if !Uuid::parse_str(&device_id).is_ok() {
            Err("Device ID not valid, should be a UUID")?
        }

        let mut data = HashMap::new();
        data.insert("channel", format!("{:?}", channel));
        data.insert("token", token);

        let resp = self
            .client
            .put(format!(
                "{}/{}/token/{}",
                self.endpoint, self.user_id, device_id
            ))
            .form(&data)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let resp: ClientResponse<String> = resp.json().await?;
            Err(resp.body)?
        }
    }
}
