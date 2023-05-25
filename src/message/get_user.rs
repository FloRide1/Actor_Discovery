use coerce::actor::message::Message;

use crate::models::users::UsersModel;

#[derive(Clone)]
pub struct GetUserMessage {
    pub user: String,
}

impl Message for GetUserMessage {
    type Result = Option<UsersModel>;
}

impl GetUserMessage {
    pub fn new(payload: String) -> Self {
        Self { user: payload }
    }
}
