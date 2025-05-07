use crate::models::contact::{Contact, NewContact};
use anyhow::Result;

pub trait ContactRepositoryTrait {
    fn create(&mut self, new_contact: NewContact) -> Result<Contact>;
    fn list_all(&mut self) -> Result<Vec<Contact>>;
    fn delete(&mut self, id: i32) -> Result<usize>;
    fn find_one(&mut self, id: i32) -> Result<Contact>;
}