use std::env;
use redis::Client;

pub fn create_redis_pool() ->Client {
    let redis_conn_string = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_conn_string).unwrap();
    client
}