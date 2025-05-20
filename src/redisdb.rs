use redis::{AsyncCommands, Client, RedisError, RedisResult};
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::Datelike;

pub struct ScoreManager {
    client: Client,
    connection: Arc<Mutex<MultiplexedConnection>>,
}

impl ScoreManager {
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = Client::open(redis_url)?;
        let connection = client.get_multiplexed_async_connection().await?;
        Ok(Self {
            client,
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn remove_member(&self, key: &str, member: &str) -> RedisResult<i64> {
        let mut conn = self.connection.lock().await;
        conn.zrem(key, member).await
    }

    pub async fn add_member(&self, key: &str, member: &str, kills: f64) -> RedisResult<i64> {
        use chrono::{Local, NaiveDate};
        use redis::AsyncCommands;

        let mut conn = self.connection.lock().await;

        // Add the member to the sorted set
        let added = conn.zadd(key, member, kills).await?;

        // Calculate the timestamp for the next midnight
        let now = Local::now().naive_local();
        let tomorrow = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
            .unwrap()
            .succ_opt()
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let ttl = tomorrow.timestamp();

        // Explicitly specify the expected return type
        conn.expire_at::<_, ()>(key, ttl).await?;

        Ok(added)
    }




    pub async fn get_top_players(&self, key: &str) -> RedisResult<Vec<(String, f64)>> {
        let mut conn = self.connection.lock().await;
        conn.zrevrange_withscores(key, 0, 99).await
    }

}
