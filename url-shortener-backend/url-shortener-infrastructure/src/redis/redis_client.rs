use async_trait::async_trait;
use coi::{Inject, Provide};
use error_stack::{Report, ResultExt};
use mockall::automock;
use redis::{AsyncCommands, Client};
use crate::redis::error::CacheError;

#[async_trait]
#[automock]
pub trait RedisClientWrapperTrait: Inject {
    async fn get_cache(&self, key: &str) -> Result<String, Report<CacheError>>;
    async fn set_cache(&self, key: &str, value: &str) -> Result<(), Report<CacheError>>;
}

#[derive(Inject)]
pub struct RedisClientWrapper(Client);


#[async_trait]
impl RedisClientWrapperTrait for RedisClientWrapper {
    async fn get_cache(&self, key: &str) -> Result<String, Report<CacheError>> {
        let mut con = self.0.get_multiplexed_tokio_connection().await
            .attach_printable_lazy(|| format!("Failed to set connection: {}", key))
            .change_context(CacheError)?;
        
        con.get(key).await
            .attach_printable_lazy(|| format!("Failed to get cache: {}", key))
            .change_context(CacheError)
    }
    
    async fn set_cache(&self, key: &str, value: &str) -> Result<(), Report<CacheError>> {
        
        let mut con = self.0.get_multiplexed_tokio_connection().await
            .attach_printable_lazy(|| format!("Failed to set connection: {}", key))
            .change_context(CacheError)?;
        
        con.set(key, value).await
            .attach_printable_lazy(|| format!("Failed to set cache: {} - {}", key, value))
            .change_context(CacheError)?;
        
        Ok(())
    }
}

#[derive(Provide)]
#[coi(provides dyn RedisClientWrapperTrait with RedisClientWrapper(self.0.clone()))]
pub struct RedisClientProvider(Client);

impl RedisClientProvider {
    pub fn new(client: Client) -> Self {
        Self(client)
    }
}

impl Inject for MockRedisClientWrapperTrait {}