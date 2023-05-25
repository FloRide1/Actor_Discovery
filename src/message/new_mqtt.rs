use coerce::actor::message::Message;

#[derive(Clone)]
pub struct NewMQTTMessage {
    pub payload: String,
}

impl Message for NewMQTTMessage {
    type Result = ();
}

impl NewMQTTMessage {
    pub fn new(payload: String) -> Self {
        Self { payload }
    }
}
