use crate::s3::error::S3Error;
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use coi::{Inject, Provide};
use error_stack::{Report, ResultExt};
use mockall::automock;
use std::env;

#[async_trait]
#[automock]
pub trait S3ClientWrapperTrait: Inject {
    async fn upload_image(&self, image: Vec<u8>, file_name: &str) -> Result<(), Report<S3Error>>;
}
#[derive(Inject)]
pub struct S3ClientWrapper(Client);

#[async_trait]
impl S3ClientWrapperTrait for S3ClientWrapper {
    async fn upload_image(&self, image: Vec<u8>, file_name: &str) -> Result<(), Report<S3Error>> {
        let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");
        let body = ByteStream::from(image);

        self.0
            .put_object()
            .bucket(bucket_name)
            .key(file_name)
            .body(body)
            .send()
            .await
            .attach_printable_lazy(|| format!("Failed to upload image: {}", file_name))
            .change_context(S3Error)?;

        Ok(())
    }
}

#[derive(Provide)]
#[coi(provides dyn S3ClientWrapperTrait with S3ClientWrapper(self.0.clone()))]
pub struct S3ClientProvider(Client);

impl S3ClientProvider {
    pub fn new(client: Client) -> Self {
        Self(client)
    }
}

// for mocking purposes
impl Inject for MockS3ClientWrapperTrait {}
