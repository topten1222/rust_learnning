use crate::models::contact::{Contact, NewContact};
use crate::schema::contacts::dsl::*;
use diesel::prelude::*;
use anyhow::Result;

use super::contact::ContactRepositoryTrait;

pub struct ContactRepository<'a> {
    pub conn: &'a mut PgConnection,
}

impl<'a> ContactRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }
}

impl<'a> ContactRepositoryTrait for ContactRepository<'a> {

    fn create(&mut self, new_contact: NewContact) -> Result<Contact> {
        diesel::insert_into(contacts)
            .values(&new_contact)
            .get_result(self.conn)
            .map_err(|e| anyhow::anyhow!(e))
    }

    fn list_all(&mut self) -> Result<Vec<Contact>> {
        contacts.load::<Contact>(self.conn).map_err(|err|anyhow::anyhow!(err))
    }
    
    fn delete(&mut self, contact_id: i32) -> Result<usize> {
        diesel::delete(contacts.find(contact_id))
            .execute(self.conn)
            .map_err(|err|anyhow::anyhow!(err))
    }

    fn find_one(&mut self, contact_id: i32) -> Result<Contact> {
        contacts.find(contact_id).first(self.conn).map_err(|err|anyhow::anyhow!(err))
    }
}