use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::schema::posts;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable, Deserialize, Validate)]
#[diesel(table_name = posts)]
pub struct NewPost {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    #[validate(length(min = 1, message = "Body is required"))]
    pub body: String,

    pub published: bool,
}
