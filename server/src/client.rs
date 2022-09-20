
use chat_manager::chat_manager_client::ChatManagerClient;

pub mod chat_manager {
    tonic::include_proto!("chat_manager");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chat_service = ChatManagerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(());

    let mut response = chat_service.get_chat_messages(request).await?.into_inner();

    loop {
        let message = response.message().await?;
        if (!message.is_none()) {
            println!("Got a message: {:?}", message);
        }
    }

    Ok(())
}
