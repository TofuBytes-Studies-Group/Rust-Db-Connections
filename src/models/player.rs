    // src/models/player.rs
    use tokio_postgres::Error;
    use crate::psqldb::Database;
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct Player {
        pub id: Uuid,
        pub name: String,
        pub health: i32,
        pub inventory_id: Option<Uuid>,
    }

    impl Player {
        pub async fn create(db: &Database, name: &str, health: i32) -> Result<Player, Error> {
            let row = db.client.query_one(
                "INSERT INTO player (name, health) VALUES ($1, $2) RETURNING id, name, health, inventory_id",
                &[&name, &health]
            ).await?;

            Ok(Player {
                id: row.get(0),
                name: row.get(1),
                health: row.get(2),
                inventory_id: row.get(3),
            })
        }

        pub async fn get_all(db: &Database) -> Result<Vec<Player>, Error> {
            let rows = db.client.query("SELECT id, name, health, inventory_id FROM player", &[]).await?;
            Ok(rows.into_iter().map(|row| Player {
                id: row.get(0),
                name: row.get(1),
                health: row.get(2),
                inventory_id: row.get(3),
            }).collect())
        }

        pub async fn get_by_id(db: &Database, id: Uuid) -> Result<Option<Player>, Error> {
            let row = db.client.query_opt(
                "SELECT id, name, health, inventory_id FROM player WHERE id = $1",
                &[&id]
            ).await?;

            match row {
                Some(row) => Ok(Some(Player {
                    id: row.get(0),
                    name: row.get(1),
                    health: row.get(2),
                    inventory_id: row.get(3),
                })),
                None => Ok(None),
            }
        }
    }
