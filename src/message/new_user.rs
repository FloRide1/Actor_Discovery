use coerce::actor::message::Message;

use crate::models::users::UsersModel;

#[derive(Clone)]
pub struct NewUserMessage {
    pub user: String,
}

impl Message for NewUserMessage {
    type Result = Result<UsersModel, diesel::result::Error>;
}

impl NewUserMessage {
    pub fn new(payload: String) -> Self {
        Self { user: payload }
    }
}
