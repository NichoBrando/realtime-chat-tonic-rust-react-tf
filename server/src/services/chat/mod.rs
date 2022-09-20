use crate::chat_manager::{
    chat_manager_server::ChatManager, ChatMessage, ConnectRequest, ConnectResponse, MessageRequest,
};
use futures::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::Sender;
mod functions;
use tonic::{Request, Response, Status};

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
pub type MessageList = Arc<RwLock<Vec<ChatMessage>>>;
pub type ReceiverList = Arc<RwLock<Vec<Sender<Result<ChatMessage, Status>>>>>;
pub type UserMap = Arc<RwLock<HashMap<String, String>>>;

#[derive(Debug, Default)]
pub struct ChatService {
    pub messages: MessageList,
    pub receivers: ReceiverList,
    pub user_map: UserMap,
}

impl ChatService {
    pub async fn push_message(&self, message: ChatMessage) {
        self.messages.write().unwrap().push(message.clone());

        let receivers = { self.receivers.read().unwrap().clone() };
        let mut indexes_to_remove = Vec::new();

        let mut index = 0;
        for receiver in receivers.iter() {
            match receiver.send(Ok(message.clone())).await {
                Ok(_) => (),
                Err(_) => {
                    indexes_to_remove.push(index.clone());
                }
            };
            index += 1;
        }

        let mut receivers = self.receivers.write().unwrap();

        let mut i = 0;
        if indexes_to_remove.len() > 0 {
            for index in indexes_to_remove.iter() {
                receivers.remove(index - i);
                i += 1;
            }
        }
    }
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

    async fn send_message(&self, request: Request<MessageRequest>) -> Result<Response<()>, Status> {
        functions::send_message(self, request).await
    }
}
