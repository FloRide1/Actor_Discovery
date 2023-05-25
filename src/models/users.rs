use std::fmt;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::schema::users;

#[derive(Debug, PartialEq, Queryable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UsersModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = email)]
    pub email: String,
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Clone)]
#[diesel(table_name = users)]
pub struct NewUsers {
    #[diesel(column_name = email)]
    pub email: String,
}

impl fmt::Display for UsersModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Users {email}>", email = self.email)
    }
}

impl UsersModel {
    pub async fn list_users(
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<UsersModel>, diesel::result::Error> {
        users::table
            .load::<UsersModel>(conn)
            .await
    }

    pub async fn get_user(conn: &mut AsyncPgConnection, id: i32) -> Option<Self> {
        users::table
            .filter(users::id.eq(id))
            .first::<Self>(conn)
            .await
            .ok()
    }

    pub async fn find_user_by_email(conn: &mut AsyncPgConnection, email: &str) -> Option<Self> {
        users::table
            .filter(users::email.eq(email))
            .first::<Self>(conn)
            .await
            .ok()
    }

    pub async fn new_user(
        conn: &mut AsyncPgConnection,
        new_user: NewUsers,
    ) -> Result<UsersModel, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await
    }


    pub async fn edit_user(
        conn: &mut AsyncPgConnection,
        id: i32,
        edited_user: NewUsers,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(edited_user)
            .execute(conn)
            .await
    }

    pub async fn delete_user(conn: &mut AsyncPgConnection, id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(conn)
            .await
    }
}

impl NewUsers {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }

}
