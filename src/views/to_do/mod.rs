use actix_web::web;
use super::path::Path;
mod create;
mod delete;
mod get;
mod utils;
mod edit;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: "/item".to_string(),
        backend: true,
    };
    app.route(
        &base_path.define("/create/{title}".to_string()),
        web::post().to(create::create)
    );
    app.route(
        &base_path.define("/get".to_string()),
        web::get().to(get::get)
    );
    app.route(
        &base_path.define("/edit".to_string()),
        web::put().to(edit::edit)
    );
    app.route(
        &base_path.define("/delete".to_string()),
        web::post().to(delete::delete)
    );
}