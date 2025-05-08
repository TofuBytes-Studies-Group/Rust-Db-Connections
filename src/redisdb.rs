// src/redisdb.rs
use redis::{aio::ConnectionManager, AsyncCommands, Client, RedisError};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScore {
    pub player_name: String,
    pub kills: u32,
    pub gold: u32,
    pub game_time: u32,
}

#[derive(Debug, Error)]
pub enum GameScoreError {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] SerdeError),
}

pub struct RedisDatabase {
    manager: ConnectionManager,
}

impl RedisDatabase {
    pub async fn connect() -> Result<Self, RedisError> {
        let client = Client::open("redis://127.0.0.1/")?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    pub async fn save_score(&mut self, player_name: &str, score: GameScore) -> Result<(), GameScoreError> {
        let key = format!("player:score:{}", player_name);
        let score_json = serde_json::to_string(&score)?;

        // Note the lifetime 'static added here
        self.manager.set(key, score_json).await.map_err(GameScoreError::Redis)
    }


    pub async fn get_score(&mut self, player_name: &str) -> Result<Option<GameScore>, GameScoreError> {
        let key = format!("player:score:{}", player_name);
        let score_json: Option<String> = self.manager.get(key).await?;
        match score_json {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    pub async fn delete_score(&mut self, player_name: &str) -> Result<(), GameScoreError> {
        let key = format!("player:score:{}", player_name);
        let _: u64 = self.manager.del(key).await.map_err(GameScoreError::Redis)?;
        Ok(())
    }
}
