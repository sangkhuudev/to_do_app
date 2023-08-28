use actix_web::web;
pub mod login;
mod logout;
use super::path::Path;

/// This function adds the auth views to the web server.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None
pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/auth"),
        backend: true
    };
    let app = app.route(
        &base_path.define("/login".to_string()),
        web::post().to(login::login),
    );
    app.route(
        &base_path.define("/logout".to_string()),
        web::post().to(logout::logout),
    );
}
