use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::contacts;

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = contacts)]
pub struct Contact {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub files: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = contacts)]
pub struct NewContact {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub files: Option<String>
}