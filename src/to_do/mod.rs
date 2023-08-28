pub mod structs;

use structs::{done::Done, pending::Pending};

pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
}

pub fn to_do_factory(item_type: &str, item_title: String) -> Result<ItemTypes, &'static str> {
    match item_type {
        "pending" => {
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        },
        "done" => {
            let done_item = Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        },
        _ => Err("This is not accepted.")
    }
}
