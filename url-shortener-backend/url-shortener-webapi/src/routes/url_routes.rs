use crate::handlers::url_handler::{create_url, get_url};
use actix_web::web;

pub(crate) fn register_url_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/url").service(create_url));

    cfg.service(get_url);
}
