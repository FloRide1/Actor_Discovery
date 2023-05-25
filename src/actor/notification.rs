use coerce::actor::{Actor, context::ActorContext, message::Handler};
use async_trait::async_trait;

use crate::message::new_notification::NewNotificationMessage;

#[derive(Default)]
pub struct NotificationActor;

#[async_trait]
impl Actor for NotificationActor {
    async fn started(&mut self, _ctx: &mut ActorContext) {
        info!("NotificationActor has been created");
    }
}

#[async_trait::async_trait]
impl Handler<NewNotificationMessage> for NotificationActor {
    async fn handle(&mut self, msg: NewNotificationMessage, _ctx: &mut ActorContext) {
        println!("[NOTIFICATION]: '{}'", msg.notification);
    }
}
