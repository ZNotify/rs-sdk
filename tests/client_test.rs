use znotify::{Client, MessageOptions};

#[tokio::test]
async fn test_client_create_err() {
    let client = Client::create("error".to_string(), None).await;
    assert!(client.is_err());
    assert!(client.err().unwrap().to_string().contains("User ID not valid"));
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
            priority: None,
        })
        .await;
    assert!(message.is_ok());
    let message = message.unwrap();
    assert_eq!(message.content, content);
    assert_eq!(message.title, title.unwrap());
    assert_eq!(message.long, long.unwrap());
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
    assert!(message.err().unwrap().to_string().contains("Content is required"));
}