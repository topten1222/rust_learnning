use diesel::{PgConnection, RunQueryDsl};
use crate::models::contact::{Contact, NewContact};
use crate::schema::contacts::dsl::*;
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
    
}