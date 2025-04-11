use async_trait::async_trait;
use coi::{Inject, Provide};
use mockall::automock;
use redis::{AsyncCommands, Client, RedisResult};

#[automock]
#[async_trait]
pub trait RedisClientWrapperTrait: Inject {
    async fn get_cache(&self, keys: &str) -> RedisResult<String>;
    async fn set_cache(&self, key: &str, value: &str) -> RedisResult<()>;
}

#[derive(Inject)]
pub struct RedisClientWrapper(Client);


#[async_trait]
impl RedisClientWrapperTrait for RedisClientWrapper {
    async fn get_cache(&self, keys: &str) -> RedisResult<String> {
        let mut con = self.0.get_multiplexed_tokio_connection().await?;
        con.get(keys).await
    }
    
    async fn set_cache(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut con = self.0.get_multiplexed_tokio_connection().await?;
        con.set(key, value).await
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