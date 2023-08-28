use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;
use actix_web::{Responder, HttpResponse, HttpRequest, Error};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_buffer = Vec::new();
        let mut done_buffer = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_buffer.push(packed.super_struct)
            }
        }
        let done_count: i8 = done_buffer.len() as i8;
        let pending_count: i8 = pending_buffer.len() as i8;
        ToDoItems {
            pending_items: pending_buffer,
            done_items: done_buffer ,
            pending_item_count: pending_count,                      
            done_item_count: done_count,
        }
    }
}

impl Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>; 
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        let response = HttpResponse::Ok().body(body);
        ready(Ok(response))
    }
}
