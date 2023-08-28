use super::base::Base;
use super::traits::{get::Get, edit::Edit, delete::Delete, create::Create};


/// This struct defines a to do item for a Pending to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Pending {
    pub super_struct: Base
}

impl Pending {

     /// The constructor for the Done struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    ///
    /// # Returns
    /// (Pending): the constructed Pending struct
    pub fn new(input_title: String) -> Self {
        let base: Base = Base::new(input_title, "pending".to_string());
        Pending { super_struct: base }
    }
}
impl Get for Pending {}
impl Edit for Pending {}  
impl Delete for Pending {}
impl Create for Pending {}

#[cfg(test)]
mod pending_test {

    use super::Pending;

    #[test]
    fn new() {
        let expected_status: String = String::from("pending");
        let title: String = String::from("washing");
        let expected_title: String = String::from("washing");

        let done: Pending = Pending::new(title);
        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }

}
