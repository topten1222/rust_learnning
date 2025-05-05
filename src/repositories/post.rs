use crate::models::post::{NewPost, Post};
use anyhow::Result;

pub trait PostRepositoryTrait {
    fn find_by_id(&mut self, pid: i32) -> Result<Post>;
    fn list_all(&mut self) -> Result<Vec<Post>>;
    fn create(&mut self, new_post: NewPost) -> Result<Post>;
    fn update(&mut self, new_post: NewPost) -> Result<Post>;
    fn delete(&mut self, pid: i32) -> Result<usize>;
}