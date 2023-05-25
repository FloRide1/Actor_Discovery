use coerce::actor::{Actor, context::ActorContext, message::Handler, LocalActorRef};
use async_trait::async_trait;
use serde_json::Value;
use regex::Regex;
use lazy_static::lazy_static;

use crate::{message::{new_mqtt::NewMQTTMessage, new_user::NewUserMessage, get_user::GetUserMessage, new_notification::NewNotificationMessage}, error::parser_error::{EmailParserError, ContentParserError}, models::users::UsersModel};

use super::{user::UserActor, notification::NotificationActor};

pub struct ParserActor {
    user_addr: LocalActorRef<UserActor>,
    notification_addr: LocalActorRef<NotificationActor>,
}

impl ParserActor {
    pub fn new(user_addr: LocalActorRef<UserActor>, notification_addr: LocalActorRef<NotificationActor>) -> Self {
        Self {
            user_addr,
            notification_addr
        }
    }

}

#[async_trait]
impl Actor for ParserActor {
    async fn started(&mut self, _ctx: &mut ActorContext) {
        info!("ParserActor has been created");
    }
}

#[async_trait::async_trait]
impl Handler<NewMQTTMessage> for ParserActor {
    async fn handle(&mut self, msg: NewMQTTMessage, _ctx: &mut ActorContext) {
        debug!("ParserActor receive a new message: '{}'", msg.payload);

        let message = serde_json::from_str::<Value>(&msg.payload);
        if let Err(err) = message {
            error!("Parser Error for \"{}\": \"{}\"", msg.payload, err);
            return;
        }

        let message = message.unwrap();

        let email = validate_email(&message);
        let content = validate_content(&message);

        if let Err(err) = email {
            error!("Parser Error email cannot be used: {}", err);
            return;
        }
        let email = email.unwrap();
        trace!("Email: {}", email);

        if let Err(err) = content {
            error!("Content Error cannot be used: {}", err);
            return;
        }
        let content = content.unwrap();
        trace!("Content: {}", email);

        let user_opt = self.user_addr.send(GetUserMessage::new(email.to_string())).await.unwrap();
        let user: UsersModel;
        match user_opt {
            Some(u) => user = u,
            None => user = self.user_addr.send(NewUserMessage::new(email.to_string())).await.unwrap().unwrap(),
        }

        println!("{:?}", user);
        let _ = self.notification_addr.notify(NewNotificationMessage::new(format!("{}: {}", email, content)));
    }
}

fn validate_email(message: &Value) -> Result<String, EmailParserError> {
    if message["email"].is_null() {
        return Err(EmailParserError::IsNullValue);
    }
    if !message["email"].is_string() {        
        return Err(EmailParserError::IsNotString);
    }

    let email = message["email"].as_str().unwrap();

    lazy_static! {
        static ref EMAIL_REGEX: Regex = Regex::new("^[\\w\\-\\.]+@([\\w-]+\\.)+[\\w-]{2,}$").unwrap();
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(EmailParserError::IsInvalid(email.to_string()));
    }

    Ok(email.to_string())
}

fn validate_content(message: &Value) -> Result<String, ContentParserError> {
    if message["content"].is_null() {
        return Err(ContentParserError::IsNullValue);
    }
    if !message["content"].is_string() {
        return Err(ContentParserError::IsNotString);
    }

    let content = message["content"].as_str().unwrap();

    Ok(content.to_string())
}
