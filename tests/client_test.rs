use znotify::{Client, MessageOptions};
use znotify::entity::{Channel, Priority};

#[tokio::test]
async fn test_client_create_err() {
    let client = Client::create("error".to_string(), None).await;
    assert!(client.is_err());
    assert_eq!("User secret not valid", client
        .err()
        .unwrap()
        .to_string())
}

#[tokio::test]
async fn test_client_create_ok() {
    let client = Client::create("test".to_string(), None).await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn client_send() {
    let content = "test".to_string();
    let title = Some("test_title".to_string());
    let long = Some("test_long".to_string());
    let client = Client::create("test".to_string(), None).await.unwrap();
    let message = client
        .send(MessageOptions {
            content: content.clone(),
            title: title.clone(),
            long: long.clone(),
            priority: Some(Priority::High),
        })
        .await;
    assert!(message.is_ok());
    let message = message.unwrap();
    assert_eq!(message.content, content);
    assert_eq!(message.title, title.unwrap());
    assert_eq!(message.long, long.unwrap());
    assert_eq!(message.priority, Priority::High);
}

#[tokio::test]
async fn client_send_failed() {
    let content = "".to_string();
    let title = Some("test_title".to_string());
    let long = Some("test_long".to_string());
    let client = Client::create("test".to_string(), None).await.unwrap();
    let message = client
        .send(MessageOptions {
            content: content.clone(),
            title: title.clone(),
            long: long.clone(),
            priority: None,
        })
        .await;
    assert!(message.is_err());
    assert!(message
        .err()
        .unwrap()
        .to_string()
        .contains("Content is required"));
}

#[tokio::test]
async fn client_delete() {
    let content = "test".to_string();
    let title = Some("test_title".to_string());
    let long = Some("test_long".to_string());
    let client = Client::create("test".to_string(), None).await.unwrap();
    let message = client
        .send(MessageOptions {
            content: content.clone(),
            title: title.clone(),
            long: long.clone(),
            priority: Some(Priority::High),
        })
        .await;
    assert!(message.is_ok());
    let message = message.unwrap();
    let id = message.id;

    let ret = client.delete_message(id).await;
    assert!(ret.is_ok());
    assert!(ret.unwrap());
}

#[tokio::test]
async fn client_create_device() {
    let client = Client::create("test".to_string(), None).await.unwrap();
    let ret = client.create_device(
        Channel::FCM,
        "test".to_string(),
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    ).await;
    assert!(ret.is_err());
    assert_eq!(ret.unwrap_err().to_string(), "Device ID not valid, should be a UUID");

    let ret2 = client.create_device(
        Channel::FCM,
        "9a31666d-1b9e-4d9b-ba63-53c9f381e52b".to_string(),
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    ).await;
    assert!(ret2.is_ok());
    assert!(ret2.unwrap());
}
