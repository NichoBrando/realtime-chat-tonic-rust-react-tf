use chat_manager::chat_manager_server::ChatManagerServer;
use tonic::transport::Server;
use services::ChatService;

mod services;

pub mod chat_manager {
    tonic::include_proto!("chat_manager");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let chat_service = ChatManagerServer::new(ChatService::default());

    Server::builder()
        .add_service(chat_service)
        .serve(addr)
        .await?;

    Ok(())
}
