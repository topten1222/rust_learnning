use crate::models::post::{Post, NewPost};
use diesel::prelude::*;
use crate::schema::posts::dsl::*;
use anyhow::Result;

use super::post::PostRepositoryTrait;

pub struct PostRepository<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> PostRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl<'a> PostRepositoryTrait for PostRepository<'a> {

    fn list_all(&mut self) -> Result<Vec<Post>> {
        posts.load::<Post>(self.conn).map_err(|e| anyhow::anyhow!(e))
    }

    fn find_by_id(&mut self, pid: i32) -> Result<Post> {
        posts.find(pid).first(self.conn).map_err(|e| anyhow::anyhow!(e))
    }

    fn create(&mut self, new_post: NewPost) -> Result<Post> {
        diesel::insert_into(posts)
            .values(&new_post)
            .get_result(self.conn)
            .map_err(|e| anyhow::anyhow!(e))
    }
    
    fn update(&mut self, new_post: NewPost) -> Result<Post> {
        diesel::update(posts.find(new_post.id.unwrap()))
            .set((
                title.eq(new_post.title),
                body.eq(new_post.body),
                published.eq(new_post.published),
            ))
            .get_result(self.conn)
            .map_err(|e|anyhow::anyhow!(e))
    }

    fn delete(&mut self, pid: i32) -> Result<usize> {
        diesel::delete(posts.find(pid))
            .execute(self.conn)
            .map_err(|e| anyhow::anyhow!(e))
    }
}
