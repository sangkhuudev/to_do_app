use super::base::Base;
use super::traits::{get::Get, edit::Edit, delete::Delete};



/// This struct defines a to do item for a done to do item.
///
/// # Attributes
/// * super_struct (Base): Inherited struct for housing key attributes
pub struct Done {
    pub super_struct: Base
}

impl Done {
    /// The constructor for the Done struct.
    ///
    /// # Arguments
    /// * input_title (String): the title of the to do item
    ///
    /// # Returns
    /// (Done): the constructed Done struct
    pub fn new(input_title: String) -> Self {
        // let input_status = String::from("done");
        let base: Base = Base::new(input_title, "done".to_string());
        Done { super_struct: base }
    }
}
impl Get for Done {}
impl Edit for Done {}  
impl Delete for Done {}

#[cfg(test)]
mod done_test {
    use super::Done;

    #[test]
    fn new() {
        let expected_status = String::from("done");
        let expected_title = String::from("washing");
        let title = "washing".to_string();
        let done = Done::new(title);
        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}