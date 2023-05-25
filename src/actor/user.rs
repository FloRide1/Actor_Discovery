use coerce::actor::{Actor, context::ActorContext, message::Handler};
use async_trait::async_trait;

use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{message::{new_user::NewUserMessage, get_user::GetUserMessage}, models::users::{UsersModel, NewUsers}};

#[derive(Default)]
pub struct UserActor {
    db_connection: Option<AsyncPgConnection>,
}

#[async_trait]
impl Actor for UserActor {
    async fn started(&mut self, _ctx: &mut ActorContext) {
        info!("UserActor has been created");

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        self.db_connection = Some(AsyncConnection::establish(&database_url).await.unwrap());
    }
}

#[async_trait::async_trait]
impl Handler<NewUserMessage> for UserActor {
    async fn handle(&mut self, msg: NewUserMessage, _ctx: &mut ActorContext) -> Result<UsersModel, diesel::result::Error> {
        debug!("User receive a new user: '{}'", msg.user);

        let conn = self.db_connection.as_mut().unwrap();
        UsersModel::new_user(conn, NewUsers::new(&msg.user)).await
    }
}

#[async_trait::async_trait]
impl Handler<GetUserMessage> for UserActor {
    async fn handle(&mut self, msg: GetUserMessage, _ctx: &mut ActorContext) -> Option<UsersModel> {
        debug!("Trying to find user: '{}'", msg.user);

        let conn = self.db_connection.as_mut().unwrap();
        UsersModel::find_user_by_email(conn, &msg.user).await
    }
}
