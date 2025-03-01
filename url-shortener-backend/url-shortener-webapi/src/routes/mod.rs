use actix_web::web;

pub(crate) mod url_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    url_routes::register_url_routes(cfg);
}
