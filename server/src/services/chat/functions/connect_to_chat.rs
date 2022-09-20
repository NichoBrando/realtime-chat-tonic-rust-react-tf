use crate::{
    chat_manager::{ConnectRequest, ConnectResponse},
    services::chat::ChatService,
};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub async fn connect_to_chat(
    service: &ChatService,
    mut request: Request<ConnectRequest>,
) -> Result<Response<ConnectResponse>, Status> {
    let user_name = request.get_ref().user_name.clone().trim().to_string();

    if user_name.is_empty() {
        return Err(Status::invalid_argument("User name is empty"));
    }

    if user_name.len() < 3 || user_name.len() > 20 {
        return Err(Status::invalid_argument("Invalid user name"));
    }

    let reply = ConnectResponse {
        user_id: Uuid::new_v4().to_string(),
    };

    let metadata = request.metadata_mut();

    metadata.insert("user_id", reply.user_id.parse().unwrap());

    service
        .user_map
        .write()
        .unwrap()
        .insert(reply.user_id.clone(), user_name);

    Ok(Response::new(reply))
}
