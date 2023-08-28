use crate::diesel;
use diesel::prelude::*;
use crate::schema::to_do;
use crate::models::item::item::Item;
use crate::database::establish_connection;

use crate::to_do::to_do_factory;
use crate::serialization::to_do_items::ToDoItems;


/// Gets all the to do items from the state JSON file and processes them to be serialized.
///
/// # Arguments
/// user_id (&i32): the user id belonging to the request
///
/// # Returns
/// * (ToDoItems): to do items sorted into Done and Pending with count numbers
pub fn return_state(user_id: &i32) -> ToDoItems {
    let connection = establish_connection();
    let items = to_do::table
        .order(to_do::columns::id.asc())
        .filter(to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&connection).unwrap();
    let mut buffer = Vec::new();
    for item in items {
        let item = to_do_factory(&item.status, item.title).unwrap();
        buffer.push(item);
    }
    ToDoItems::new(buffer)
}