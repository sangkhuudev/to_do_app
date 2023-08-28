use actix_web::web;
mod auth;
mod path;
mod to_do;
mod app;
mod users;


pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
    app::app_factory(app);
    users::user_factory(app);
}