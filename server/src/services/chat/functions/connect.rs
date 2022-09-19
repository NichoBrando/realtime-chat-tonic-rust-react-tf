use crate::{
    services::chat::ChatService,
    chat_manager::{
        ConnectRequest,
        ConnectResponse
    }
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub async fn connect_to_chat(
    _service: &ChatService,
    mut request: Request<ConnectRequest>,
) -> Result<Response<ConnectResponse>, Status> {
    println!("Got a request: {:?}", request);

    let reply = ConnectResponse {
        user_id: Uuid::new_v4().to_string()
    };

    let metadata = request.metadata_mut();

    metadata.insert("user_id", reply.user_id.parse().unwrap());

    Ok(Response::new(reply))
}