use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::user;

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl<'a> NewUser<'a> {
    pub fn new(user_id: &'a str, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> Self {
        NewUser {
            user_id,
            created_at,
            updated_at,
        }
    }
}
