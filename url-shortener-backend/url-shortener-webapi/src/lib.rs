use crate::routes::init_routes;
use actix_web::web;

mod handlers;
mod implementations;
mod models;
mod routes;

pub fn register_api(cfg: &mut web::ServiceConfig) {
    init_routes(cfg);
}
