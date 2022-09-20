use crate::{chat_manager::ChatMessage, services::chat::ChatService};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Status};

pub async fn get_chat_messages(
    service: &ChatService,
    _request: Request<()>,
) -> Result<ReceiverStream<Result<ChatMessage, Status>>, Status> {
    let message_list = { service.messages.read().unwrap().clone() };

    let (tx, rx) = tokio::sync::mpsc::channel(1);

    for message in message_list {
        tx.send(Ok(message)).await.unwrap();
    }

    let stream_receiver = ReceiverStream::new(rx);
    service.receivers.write().unwrap().push(tx);

    Ok(stream_receiver)
}
