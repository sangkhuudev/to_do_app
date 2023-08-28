pub mod jwt;
mod processes;

use actix_web::dev::ServiceRequest;

pub fn process_token(req: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(req) {
        Ok(token) => {
            match processes::check_password(token) {
                Ok(token) => Ok(token),
                Err(message) => Err(message)
            }
        }
        Err(message) => Err(message)
    }
}

#[cfg(test)]
mod process_token_tests {
    use super::process_token;
    use super::jwt::JwtToken;
    use actix_web::test::TestRequest;

    #[test]
    fn no_token_process_token() {
        let mock_req = TestRequest::with_header("test", "test").to_srv_request();
        match process_token(&mock_req) {
            Err(message) => assert_eq!("There is no token", message),
            _ => panic!("No token in request header should fail")
        }
    }

    #[test]
    fn incorrect_token() {
        let mock_req = TestRequest::with_header("user-token", "test").to_srv_request();
        match process_token(&mock_req) {
            Err(message) => assert_eq!("Could not decode!", message),
            _ => panic!("Incorrect token should error")
        }
    }

    #[test]
    fn correct_token() {
        let encoded_token: String = JwtToken::encode(32);
        let mock_request = TestRequest::with_header("user-token", encoded_token).to_srv_request();

        match process_token(&mock_request) {
            Ok(token) => assert_eq!("passed", token),
            _ => panic!("encoded token should pass")
        }
    }    
}