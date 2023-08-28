use actix_web::web;
mod items;
mod content_loader;
use super::path::Path;
mod login;
mod logout;

/// This function adds the app views to the web server serving HTML.
///
/// # Arguments
/// * (&mut web::ServiceConfig): reference to the app for configuration
///
/// # Returns
/// None

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/"), backend: false};
    app.route(&base_path.define(String::from("")),
        web::get().to(items::items)
    );
    app.route(&base_path.define(String::from("login/")),
        web::get().to(login::login)
    );
    app.route(&base_path.define(String::from("logout/")),
        web::get().to(logout::logout)
    );
}