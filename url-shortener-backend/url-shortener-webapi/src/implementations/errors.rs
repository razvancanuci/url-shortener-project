use actix_web::http::StatusCode;
use url_shortener_application::models::errors::ApiError;

pub trait FormatErrorTrait {
    fn get_message_status(&self) -> (StatusCode, String);
}

impl FormatErrorTrait for ApiError {
    fn get_message_status(&self) -> (StatusCode, String) {
        match *self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.to_owned()),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message.to_owned()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        }
    }
}
