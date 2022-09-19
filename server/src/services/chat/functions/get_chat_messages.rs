use crate::{chat_manager::ChatMessage, services::chat::ChatService};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Status};

pub async fn get_chat_messages(
    service: &ChatService,
    _request: Request<()>,
) -> Result<ReceiverStream<Result<ChatMessage, Status>>, Status> {
    let mut message_list = { service.messages.read().unwrap().clone() };

    message_list.push(ChatMessage {
        sender: "test".to_string(),
        msg: "test".to_string(),
    });

    let (tx, rx) = tokio::sync::mpsc::channel(4);

    for message in message_list {
        tx.send(Ok(message)).await.unwrap();
    }

    tokio::spawn(async move {
        loop {
            let message = ChatMessage {
                sender: "test".to_string(),
                msg: "test".to_string(),
            };

            tx.send(Ok(message)).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let stream_receiver = ReceiverStream::new(rx);
    service.receivers.write().unwrap().push(1);

    Ok(stream_receiver)
}
