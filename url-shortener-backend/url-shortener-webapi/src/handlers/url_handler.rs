use crate::implementations::errors::FormatErrorTrait;
use crate::models::api_response_model::ApiResponseModel;
use actix_web::{get, post, web, HttpResponse};
use coi_actix_web::inject;
use url_shortener_application::models::response_model::CreateResponseModel;
use url_shortener_application::models::url_models::CreateUrlRequest;
use url_shortener_application::services::url_service::UrlServiceTrait;

#[post("")]
#[inject]
pub async fn create_url(
    request: web::Json<CreateUrlRequest>,
    #[inject] url_service: Arc<dyn UrlServiceTrait>,
) -> HttpResponse {
    let result = url_service.create_short_url(request.0).await;

    match result {
        Ok(res) => HttpResponse::Created()
            .json(ApiResponseModel::<CreateResponseModel>::success(Some(res))),
        Err(e) => {
            let (status, e) = e.get_message_status();
            HttpResponse::build(status).json(ApiResponseModel::<String>::failure(Some(e)))
        }
    }
}

#[get("/{short_url}")]
#[inject]
pub async fn get_url(
    short_url: web::Path<String>,
    #[inject] url_service: Arc<dyn UrlServiceTrait>,
) -> HttpResponse {
    let result = url_service.get_long_url(short_url.as_str()).await;
    match result {
        Ok(res) => HttpResponse::Found()
            .append_header(("Location", res))
            .finish(),
        Err(e) => {
            let (status, e) = e.get_message_status();
            HttpResponse::build(status).json(ApiResponseModel::<String>::failure(Some(e)))
        }
    }
}
