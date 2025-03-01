use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponseModel {
    #[serde(rename = "shortUrl")]
    pub short_url: String,
    #[serde(rename = "qrCodeImage")]
    pub qr_code_image: String,
}
