use coerce::actor::{Actor, context::ActorContext, message::Handler};
use async_trait::async_trait;

use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::{message::new_mqtt::NewMQTTMessage, models::persistence_log::{PersistenceLogModel, NewPersistenceLog}};

#[derive(Default)]
pub struct PersistenceActor {
    db_connection: Option<AsyncPgConnection>,
}

#[async_trait]
impl Actor for PersistenceActor {
    async fn started(&mut self, _ctx: &mut ActorContext) {
        info!("PersistenceActor has been created");

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        self.db_connection = Some(AsyncConnection::establish(&database_url).await.unwrap());
    }
}

#[async_trait::async_trait]
impl Handler<NewMQTTMessage> for PersistenceActor {
    async fn handle(&mut self, msg: NewMQTTMessage, _ctx: &mut ActorContext) {
        debug!("PersistenceActor receive a new message: '{}'", msg.payload);

        let conn = self.db_connection.as_mut().unwrap();
        let res = PersistenceLogModel::new_log(conn, NewPersistenceLog::new(&msg.payload)).await;
        
        if let Err(err) = res {
            error!("{}", err);
            return;
        }
    }
}
