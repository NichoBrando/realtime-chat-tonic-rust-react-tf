use chat_manager::chat_manager_client::ChatManagerClient;

pub mod chat_manager {
    tonic::include_proto!("chat_manager");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let mut chat_service = ChatManagerClient::connect("http://[::1]:50051")
            .await
            .unwrap();

        let connection_request = tonic::Request::new(chat_manager::ConnectRequest {
            user_name: "TestUser".to_string(),
        });

        let connection_response = chat_service
            .connect_to_chat(connection_request)
            .await
            .unwrap()
            .into_inner();


        let mut message_number = 1;
        loop {
            let mut message_request = tonic::Request::new(chat_manager::MessageRequest {
                message: format!("Test message {}", message_number),
            });

            message_request
                .metadata_mut()
                .insert("user_id", connection_response.user_id.parse().unwrap());

            chat_service
                .send_message(message_request)
                .await
                .unwrap()
                .into_inner();

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            message_number += 1;
        }
    });

    let mut chat_service = ChatManagerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(());

    let mut response = chat_service.get_chat_messages(request).await?.into_inner();

    loop {
        let message = response.message().await?;
        if message.is_some() {
            println!("Got a message: {:?}", message.unwrap().msg);
        }
    }

    Ok(())
}
