use crate::models::contact::{Contact, NewContact};
use anyhow::Result;

pub trait ContactRepositoryTrait {
    fn create(&mut self, new_contact: NewContact) -> Result<Contact>;
}