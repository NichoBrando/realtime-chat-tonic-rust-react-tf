use crate::{
    chat_manager::{MessageRequest, ChatMessage},
    services::chat::ChatService,
};
use tonic::{Request, Response, Status};

pub async fn send_message(
    service: &ChatService,
    mut request: Request<MessageRequest>,
) -> Result<Response<()>, Status> {
    match request.metadata().get("user_id") {
        Some(user_id) => {
            
            let message = request.get_ref().message.clone().trim().to_string();

            if message.is_empty() {
                // Err(Status::invalid_argument("Message is empty"));
            }

            if message.len() > 100 {
                // Err(Status::invalid_argument("Message is too long"));
            }

            let user_id = user_id.to_str().unwrap().to_string();

            let user_name = {
                service.user_map.read().unwrap().get(&user_id).unwrap().clone()
            };

            let new_message = ChatMessage {
                sender: user_name,
                msg: message
            };

            service.push_message(&new_message).await;

            Ok(Response::new(()))
        }
        None => Err(Status::unauthenticated("User is not authenticated")),
    }
}
