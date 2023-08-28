use crate::diesel;
use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::auth::jwt::JwtToken;
use super::utils::return_state;
use actix_web::{HttpRequest, Responder};

/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (impl Responder): message to be sent back to the user

pub async fn create(req: HttpRequest) -> impl Responder {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let tilte_reference = req.match_info().get("title").unwrap().to_string();
    let connection = establish_connection();
    let token = JwtToken::decode_from_request(req).unwrap();
    let items = to_do::table
        .filter(to_do::columns::title.eq(tilte_reference.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id.clone());
        let _ = diesel::insert_into(to_do::table).values(&new_post).execute(&connection);
    }
    return_state(&token.user_id)
}