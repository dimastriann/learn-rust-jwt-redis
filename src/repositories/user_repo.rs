use diesel::prelude::*;
use crate::schema::{users, contacts};
use crate::models::user::{User, NewUser, UpdateUser, Contact, NewContact, UpdateContact};
use crate::db::DbPooledConn as DbConn;

pub struct UserRepository;

impl UserRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<User>> {
        users::table.select(User::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<User> {
        users::table.find(id).first(conn)
    }

    pub fn create(conn: &mut DbConn, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn update(conn: &mut DbConn, user: UpdateUser) -> QueryResult<User> {
        diesel::update(users::table.find(user.id))
            .set(&user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<User> {
        diesel::delete(users::table.find(id))
            .get_result(conn)
    }
}

pub struct ContactRepository;

impl ContactRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<Contact>> {
        contacts::table.select(Contact::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<Contact> {
        contacts::table.find(id).first(conn)
    }

    pub fn create(conn: &mut DbConn, new_contacts: Vec<NewContact>) -> QueryResult<Vec<Contact>> {
        diesel::insert_into(contacts::table)
            .values(&new_contacts)
            .returning(Contact::as_returning())
            .get_results(conn)
    }

    pub fn update(conn: &mut DbConn, contact: UpdateContact) -> QueryResult<Contact> {
        diesel::update(contacts::table.find(contact.id))
            .set(&contact)
            .returning(Contact::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<Contact> {
        diesel::delete(contacts::table.find(id))
            .get_result(conn)
    }
}
