use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use coi::container;
use coi_actix_web::AppExt;
use dotenv::dotenv;
use std::env;
use url_shortener_application::services::url_service::UrlServiceProvider;
use url_shortener_database::database::pool::{crete_database_connection, PgPoolProvider};
use url_shortener_database::repositories::url_repository::UrlRepositoryProvider;
use url_shortener_infrastructure::s3::config::create_s3_client;
use url_shortener_infrastructure::s3::s3_client::S3ClientProvider;
use url_shortener_webapi::register_api;

const MAX_REQUEST_PER_SEC_ALLOWED: u32 = 10;
const SECONDS_PER_REQUEST: u64 = 3;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pg_pool = crete_database_connection()
        .await
        .expect("Failed to connect to database");
    let db = PgPoolProvider::new(pg_pool);
    let s3_client = create_s3_client().await;
    let s3_client_wrapper = S3ClientProvider::new(s3_client);

    let container = container! {
        s3_client_wrapper => s3_client_wrapper; singleton,
        db => db; singleton,
        url_service => UrlServiceProvider; scoped,
        url_repository => UrlRepositoryProvider; scoped,
    };

    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(SECONDS_PER_REQUEST)
        .burst_size(MAX_REQUEST_PER_SEC_ALLOWED)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(
                env::var("FRONTEND_URL")
                    .expect("FRONTEND_URL must be set")
                    .as_str(),
            )
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .register_container(container.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Governor::new(&governor_conf))
            .configure(register_api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
