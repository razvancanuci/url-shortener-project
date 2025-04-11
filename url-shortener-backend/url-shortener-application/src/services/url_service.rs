use crate::models::errors::ApiError;
use crate::models::response_model::CreateResponseModel;
use crate::models::url_models::CreateUrlRequest;
use async_trait::async_trait;
use coi::Inject;
use log::{error, warn};
use qrcode_generator::QrCodeEcc;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::sync::Arc;
use url::Url;
use url_shortener_database::repositories::url_repository::UrlRepositoryTrait;
use url_shortener_infrastructure::redis::redis_client::{RedisClientWrapperTrait};
use url_shortener_infrastructure::s3::s3_client::S3ClientWrapperTrait;

const CODE_LENGTH: usize = 6;
const QR_CODE_SIZE: usize = 256;
#[async_trait]
pub trait UrlServiceTrait: Inject {
    async fn create_short_url(
        &self,
        create_url_request: CreateUrlRequest,
    ) -> Result<CreateResponseModel, ApiError>;
    async fn get_long_url(&self, short_url: &str) -> Result<String, ApiError>;
}

#[derive(Inject)]
#[coi(provides pub dyn UrlServiceTrait with UrlService::new(url_repository, s3_client_wrapper, redis_client_wrapper))]
struct UrlService {
    #[coi(inject)]
    url_repository: Arc<dyn UrlRepositoryTrait>,
    #[coi(inject)]
    s3_client_wrapper: Arc<dyn S3ClientWrapperTrait>,
    #[coi(inject)]
    redis_client_wrapper: Arc<dyn RedisClientWrapperTrait>
}

impl UrlService {
    pub fn new(
        url_repository: Arc<dyn UrlRepositoryTrait>,
        s3_client_wrapper: Arc<dyn S3ClientWrapperTrait>,
        redis_client_wrapper: Arc<dyn RedisClientWrapperTrait>
    ) -> Self {
        Self {
            url_repository,
            s3_client_wrapper,
            redis_client_wrapper
        }
    }

    fn generate_short_code() -> String {
        let rng = rng();
        let short_code: String = rng
            .sample_iter(&Alphanumeric)
            .take(CODE_LENGTH)
            .map(char::from)
            .collect();
        short_code
    }

    fn validate_url(url: &str) -> Result<(), ApiError> {
        if url.is_empty() {
            warn!("Url is empty {url:?}");
            return Err(ApiError::BadRequest("Url is empty"));
        }
        match Url::parse(url) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Parsing url failed {e:?}");
                Err(ApiError::BadRequest("Invalid url"))
            }
        }
    }

    fn generate_qr_code(url: &str) -> Result<Vec<u8>, ApiError> {
        let result = qrcode_generator::to_png_to_vec(url, QrCodeEcc::Low, QR_CODE_SIZE);

        match result {
            Ok(r) => Ok(r),
            Err(e) => {
                error!("Failed to generate qr code: {:?}", e);
                Err(ApiError::InternalServerError)
            }
        }
    }
}

#[async_trait]
impl UrlServiceTrait for UrlService {
    async fn create_short_url(
        &self,
        create_url_request: CreateUrlRequest,
    ) -> Result<CreateResponseModel, ApiError> {
        let validate_url_result = Self::validate_url(&create_url_request.url);

        if validate_url_result.is_err() {
            return Err(validate_url_result.unwrap_err());
        }

        let code = Self::generate_short_code();
        let url = url_shortener_database::models::url_models::Url {
            id: code,
            url: create_url_request.url.clone(),
        };

        let result = self.url_repository.create(url).await.map_err(|e| {
            log::error!("Failed to create short url: {:?}", e);
            ApiError::InternalServerError
        })?;

        let domain = std::env::var("APP_DOMAIN").expect("APP_DOMAIN must be set");
        let url_qr = format!("{}/{}", domain, result.id);
        let qr_code = Self::generate_qr_code(&url_qr)?;
        let file_name = format!("{}.png", result.id);
        let s3_result = self
            .s3_client_wrapper
            .upload_image(qr_code, &file_name)
            .await;

        match s3_result {
            Ok(_) => {
                let cloud_front_url =
                    std::env::var("CLOUD_FRONT_URL").expect("CLOUD_FRONT_URL must be set");
                let _ = self.redis_client_wrapper.set_cache(&result.id, &result.url).await;
                Ok(CreateResponseModel {
                    short_url: format!("{}/{}", domain, result.id),
                    qr_code_image: format!("{}/{}", cloud_front_url, file_name),
                })
            }
            Err(e) => {
                log::error!("Failed to upload qr code: {:?}", e);
                Err(ApiError::InternalServerError)
            }
        }
    }

    async fn get_long_url(&self, short_url: &str) -> Result<String, ApiError> {
        
        let url_cache = self.redis_client_wrapper.get_cache(short_url).await;
        
        if url_cache.is_ok() {
            return Ok(url_cache.unwrap());
        }
        
        if  url_cache.is_err() {
            warn!("Failed to fetch url cache for {short_url}");
        } 
        
        let url = self.url_repository.find(short_url).await;
        match url {
            Ok(u) => match u {
                Some(u) => {
                    let _ = self.redis_client_wrapper.set_cache(short_url, &u.url).await;
                    Ok(u.url)
                },
                None => {
                    warn!("Short url not found: {:?}", short_url);
                    Err(ApiError::NotFound("The url with this format was not found"))
                }
            },
            Err(e) => {
                log::error!("Failed to get long url: {:?}", e);
                Err(ApiError::InternalServerError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::errors::ApiError;
    use crate::models::url_models::CreateUrlRequest;
    use crate::services::url_service::UrlServiceTrait;
    use error_stack::Report;
    use mockall::predicate::{always, eq};
    use std::env;
    use std::sync::Arc;
    use url_shortener_database::models::errors::DatabaseError;
    use url_shortener_database::models::url_models::Url;
    use url_shortener_database::repositories::url_repository::MockUrlRepositoryTrait;
    use url_shortener_infrastructure::redis::redis_client::MockRedisClientWrapperTrait;
    use url_shortener_infrastructure::s3::error::S3Error;
    use url_shortener_infrastructure::s3::s3_client::MockS3ClientWrapperTrait;
    
    const TEST_SHORT_URL: &str = "1234556";
    const TEST_VALID_URL: &str = "https://www.google.com";

    fn setup_mocks() -> (MockUrlRepositoryTrait, MockS3ClientWrapperTrait, MockRedisClientWrapperTrait) {
        let repository = MockUrlRepositoryTrait::new();
        let s3_client = MockS3ClientWrapperTrait::new();
        let redis_client = MockRedisClientWrapperTrait::new();
        (repository, s3_client, redis_client)
    }
    
    #[tokio::test]
    async fn get_long_url_returns_internal_server_error() {
        // Arrange
        let (mut repository, s3_client, redis_client) = setup_mocks();
        let s3_client = Arc::new(s3_client);
        let redis_client = Arc::new(redis_client);

        repository
            .expect_find()
            .with(eq(TEST_SHORT_URL))
            .returning(|_| Box::pin(async { Err(Report::from(DatabaseError {})) }));

        let url_service = super::UrlService::new(Arc::new(repository), s3_client, redis_client);

        // Act
        let result = url_service.get_long_url(TEST_SHORT_URL).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::InternalServerError);
    }

    #[tokio::test]
    async fn get_long_url_returns_not_found_error() {
        // Arrange
        let (mut repository, client, redis_client) = setup_mocks();
        let s3_client = Arc::new(client);
        let redis_client = Arc::new(redis_client);

        repository
            .expect_find()
            .with(eq(TEST_SHORT_URL))
            .returning(|_| Box::pin(async { Ok(None) }));

        let url_service = super::UrlService::new(Arc::new(repository), s3_client, redis_client);

        // Act
        let result = url_service.get_long_url(TEST_SHORT_URL).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ApiError::NotFound("The url with this format was not found")
        );
    }

    #[tokio::test]
    async fn get_long_url_returns_ok() {
        // Arrange
        let (mut repository, client, redis_client) = setup_mocks();
        let s3_client = Arc::new(client);
        let redis_client = Arc::new(redis_client);

        repository.expect_find().with(eq(TEST_SHORT_URL)).returning(|_| {
            Box::pin(async {
                Ok(Some(Url {
                    id: "".to_string(),
                    url: "".to_string(),
                }))
            })
        });

        let url_service = super::UrlService::new(Arc::new(repository), s3_client, redis_client);

        // Act
        let result = url_service.get_long_url(TEST_SHORT_URL).await;

        // Assert
        assert!(result.is_ok());
        assert!(result.ok().is_some());
    }
    #[tokio::test]
    async fn create_url_returns_url_empty_error() {
        // Arrange
        let (repository, s3_client, redis_client) = setup_mocks();
        let repository = Arc::new(repository);
        let s3_client = Arc::new(s3_client);
        let redis_client = Arc::new(redis_client);

        let request = CreateUrlRequest {
            url: "".to_string(),
        };
        let url_service = super::UrlService::new(repository, s3_client, redis_client);

        // Act
        let result = url_service.create_short_url(request).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::BadRequest("Url is empty"));
    }

    #[tokio::test]
    async fn crate_url_returns_invalid_url_error() {
        // Arrange
        let (repository, s3_client, redis_client) = setup_mocks();
        let repository = Arc::new(repository);
        let s3_client = Arc::new(s3_client);
        let redis_client = Arc::new(redis_client);

        let request = CreateUrlRequest {
            url: "invalid_url".to_string(),
        };
        let url_service = super::UrlService::new(repository, s3_client, redis_client);

        // Act
        let result = url_service.create_short_url(request).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::BadRequest("Invalid url"));
    }

    #[tokio::test]
    async fn create_url_on_database_returns_internal_server_error() {
        // Arrange
        let (mut repository, client, redis_client) = setup_mocks();
        let s3_client = Arc::new(client);
        let redis_client = Arc::new(redis_client);

        let request = CreateUrlRequest {
            url: TEST_VALID_URL.to_string(),
        };
        repository
            .expect_create()
            .with(always())
            .returning(|_| Box::pin(async { Err(Report::from(DatabaseError {})) }));

        let url_service = super::UrlService::new(Arc::new(repository), s3_client, redis_client);

        // Act
        let result = url_service.create_short_url(request).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::InternalServerError);
    }
    #[tokio::test]
    async fn create_url_on_s3_returns_internal_server_error() {
        // Arrange
        env::set_var("APP_DOMAIN", "yes");
        let (mut repository, mut s3_client, redis_client) = setup_mocks();
        let redis_client = Arc::new(redis_client);

        let request = CreateUrlRequest {
            url: TEST_VALID_URL.to_string(),
        };
        repository.expect_create().with(always()).returning(|_| {
            Box::pin(async {
                Ok(Url {
                    id: TEST_SHORT_URL.to_string(),
                    url: TEST_VALID_URL.to_string(),
                })
            })
        });

        s3_client
            .expect_upload_image()
            .with(always(), always())
            .returning(|_, _| Box::pin(async { Err(Report::new(S3Error {})) }));

        let url_service = super::UrlService::new(Arc::new(repository), Arc::new(s3_client), redis_client);

        // Act
        let result = url_service.create_short_url(request).await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ApiError::InternalServerError);
    }

    #[tokio::test]
    async fn create_url_returns_ok() {
        // Arrange
        env::set_var("APP_DOMAIN", "yes");
        env::set_var("CLOUD_FRONT_URL", "yes");
        let (mut repository, mut s3_client, redis_client) = setup_mocks();
        let redis_client = Arc::new(redis_client);

        let request = CreateUrlRequest {
            url: TEST_VALID_URL.to_string(),
        };
        repository.expect_create().with(always()).returning(|_| {
            Box::pin(async {
                Ok(Url {
                    id: TEST_SHORT_URL.to_string(),
                    url: TEST_VALID_URL.to_string(),
                })
            })
        });

        s3_client
            .expect_upload_image()
            .with(always(), always())
            .returning(|_, _| Box::pin(async { Ok(()) }));

        let url_service = super::UrlService::new(Arc::new(repository), Arc::new(s3_client), redis_client);

        // Act
        let result = url_service.create_short_url(request).await;

        // Assert
        assert!(result.is_ok());
    }
}
