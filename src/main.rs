pub mod message;
pub mod error;
pub mod actor;
pub mod schema;
pub mod models;

extern crate diesel;
extern crate diesel_migrations;


use diesel::prelude::*;
use crate::{diesel_migrations::MigrationHarness, actor::notification::NotificationActor};

use coerce::actor::{system::ActorSystem, IntoActor};
use crate::actor::{reader::ReaderActor, persistence::PersistenceActor, parser::ParserActor, user::UserActor};

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    // Run DB migrations
    const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!("migrations");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let mut conn = diesel::PgConnection::establish(&database_url).unwrap();

    conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
    //

    let system = ActorSystem::new();

    // Notification
    let notification_addr = NotificationActor::default()
                        .into_actor(Some("notification-actor-1"), &system)
                        .await
                        .unwrap();
    
    // User
    let user_addr = UserActor::default()
                        .into_actor(Some("user-actor-1"), &system)
                        .await
                        .unwrap();


    // Parser
    let parser_addr = ParserActor::new(user_addr, notification_addr)
                        .into_actor(Some("parser-actor-1"), &system)
                        .await
                        .unwrap();

    // Persistence
    let persistence_addr = PersistenceActor::default()
                        .into_actor(Some("persistence-actor-1"), &system)
                        .await
                        .unwrap();

    // Reader
    let _reader_addr = ReaderActor::new(persistence_addr, parser_addr)
                        .into_actor(Some("reader-actor-1"), &system)
                        .await
                        .unwrap();
    
    loop {}
}

