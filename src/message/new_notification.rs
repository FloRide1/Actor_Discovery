use coerce::actor::message::Message;

#[derive(Clone)]
pub struct NewNotificationMessage {
    pub notification: String,
}

impl Message for NewNotificationMessage {
    type Result = ();
}

impl NewNotificationMessage {
    pub fn new(payload: String) -> Self {
        Self {
            notification: payload,
        }
    }
}
