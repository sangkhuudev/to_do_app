use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String
}

impl Base {
    pub fn new(input_title: String, input_status: String) -> Base {
        return Base {title: input_title, status: input_status}
    }
}

#[cfg(test)]
mod base_tests {
    use super::Base;

    #[test]
    fn new() {
        let title = "test title".to_string();
        let expected_title = "test title".to_string();
        let status = "test status".to_string();
        let expected_status = "test status".to_string();
        let new_base_struct = Base::new(title, status);
        assert_eq!(new_base_struct.title, expected_title);
        assert_eq!(new_base_struct.status, expected_status);
    }
}