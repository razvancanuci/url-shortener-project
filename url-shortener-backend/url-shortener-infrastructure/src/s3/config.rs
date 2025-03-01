use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::config::{BehaviorVersion, Credentials};
use aws_sdk_s3::{Client, Config};
use std::env;

pub struct S3Config {
    pub access_key: String,
    pub secret_access_key: String,
    pub endpoint: String,
    pub region: String,
}

impl S3Config {
    pub fn from_env() -> Self {
        Self {
            access_key: env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY must be set"),
            secret_access_key: env::var("AWS_SECRET_KEY").expect("AWS_SECRET_KEY must be set"),
            endpoint: env::var("AWS_ENDPOINT").expect("AWS_ENDPOINT must be set"),
            region: env::var("AWS_REGION").expect("AWS_REGION must be set"),
        }
    }
}

pub async fn create_s3_client() -> Client {
    let config = S3Config::from_env();

    let credentials = Credentials::new(config.access_key, config.secret_access_key, None, None, "");

    let region_provider = RegionProviderChain::first_try(Region::new(config.region));
    let region = region_provider.region().await.unwrap();

    let config = Config::builder()
        .region(region)
        .credentials_provider(credentials)
        .endpoint_url(config.endpoint)
        .behavior_version(BehaviorVersion::latest())
        .force_path_style(true)
        .build();

    Client::from_conf(config)
}
