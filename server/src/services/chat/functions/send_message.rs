use crate::{
    chat_manager::{ChatMessage, MessageRequest},
    services::chat::ChatService,
};
use tonic::{Request, Response, Status};

pub async fn send_message(
    service: &ChatService,
    request: Request<MessageRequest>,
) -> Result<Response<()>, Status> {
    match request.metadata().get("user_id") {
        Some(user_id) => {
            let message = request.get_ref().message.clone().trim().to_string();

            if message.is_empty() {
                return Err(Status::invalid_argument("Message is empty"));
            }

            if message.len() > 100 {
                return Err(Status::invalid_argument("Message is too long"));
            }

            let user_id = user_id.to_str().unwrap().to_string();

            let user_name = {
                service
                    .user_map
                    .read()
                    .unwrap()
                    .get(&user_id)
                    .unwrap()
                    .clone()
            };

            let new_message = ChatMessage {
                sender: user_name,
                msg: message,
            };

            service.messages.write().unwrap().push(new_message.clone());

            service.push_message(new_message).await;

            return Ok(Response::new(()));
        }
        None => return Err(Status::unauthenticated("User is not authenticated")),
    };
}
