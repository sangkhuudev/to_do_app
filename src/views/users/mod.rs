mod create;
use actix_web::web;
use super::path::Path;


/// This function adds the user views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn user_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { 
        prefix: "/user".to_string(),
        backend: true,
    };

    app.route(&base_path.define("/create".to_string()),
            web::post().to(create::create)
    );
}