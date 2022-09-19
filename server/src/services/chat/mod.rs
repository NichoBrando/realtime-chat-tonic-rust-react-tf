use crate::chat_manager::{
    chat_manager_server::ChatManager, ChatMessage, ConnectRequest, ConnectResponse,
};
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
mod functions;
use tonic::{Request, Response, Status};

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
pub type MessageList = Arc<RwLock<Vec<ChatMessage>>>;
pub type ReceiverList = Arc<RwLock<Vec<i32>>>;

#[derive(Debug, Default)]
pub struct ChatService {
    pub messages: MessageList,
    pub receivers: ReceiverList,
}

#[tonic::async_trait]
impl ChatManager for ChatService {
    type get_chat_messagesStream = ResponseStream;

    async fn connect_to_chat(
        &self,
        request: Request<ConnectRequest>,
    ) -> Result<Response<ConnectResponse>, Status> {
        functions::connect_to_chat(self, request).await
    }

    async fn get_chat_messages(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::get_chat_messagesStream>, Status> {
        match functions::get_chat_messages(self, request).await {
            Ok(stream) => Ok(Response::new(
                Box::pin(stream) as Self::get_chat_messagesStream
            )),
            Err(err) => Err(err),
        }
    }
}
