use std::fmt;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::persistencelog;

#[derive(Debug, PartialEq, Queryable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = persistencelog)]
pub struct PersistenceLogModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = content)]
    pub content: String,
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Clone)]
#[diesel(table_name = persistencelog)]
pub struct NewPersistenceLog {
    #[diesel(column_name = content)]
    pub content: String,
}

impl fmt::Display for PersistenceLogModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<PersistenceLog {log}>", log = self.content)
    }
}

impl PersistenceLogModel {
    pub async fn list_logs(
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<PersistenceLogModel>, diesel::result::Error> {
        persistencelog::table
            .load::<PersistenceLogModel>(conn)
            .await
    }

    pub async fn get_log(conn: &mut AsyncPgConnection, id: i32) -> Option<Self> {
        persistencelog::table
            .filter(persistencelog::id.eq(id))
            .first::<Self>(conn)
            .await
            .ok()
    }

    pub async fn new_log(
        conn: &mut AsyncPgConnection,
        new_log: NewPersistenceLog,
    ) -> Result<PersistenceLogModel, diesel::result::Error> {
        diesel::insert_into(persistencelog::table)
            .values(new_log)
            .get_result(conn)
            .await
    }

    pub async fn delete_log(conn: &mut AsyncPgConnection, id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(persistencelog::table)
            .filter(persistencelog::id.eq(id))
            .execute(conn)
            .await
    }
}

impl NewPersistenceLog {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }

}
