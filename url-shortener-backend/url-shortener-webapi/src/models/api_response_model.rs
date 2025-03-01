use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseModel<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ApiResponseModel<T> {
    pub fn failure(message: Option<String>) -> Self {
        ApiResponseModel {
            success: false,
            value: None,
            message,
        }
    }
    pub fn success(value: Option<T>) -> Self {
        ApiResponseModel {
            success: true,
            value,
            message: None,
        }
    }
}
