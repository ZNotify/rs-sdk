use znotify::send;

#[tokio::test]
async fn client_send() {
    let content = "test".to_string();
    let title = Some("test_title".to_string());
    let long = Some("test_long".to_string());
    let message = send(
        "test".to_string(),
        content.clone(),
        title.clone(),
        long.clone(),
    )
    .await;
    assert!(message.is_ok());
    let message = message.unwrap();
    assert_eq!(message.content, content);
    assert_eq!(message.title, title.unwrap());
    assert_eq!(message.long, long.unwrap());
}
