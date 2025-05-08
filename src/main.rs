mod psqldb;
mod models;

use psqldb::Database;
use models::player::Player;
use tokio_postgres::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Database::connect().await?;

    // Create a new player
    let player = Player::create(&db, "Captain Rustbeard", 100).await?;
    println!("Created player: {:?}", player);

    // Fetch all players
    let players = Player::get_all(&db).await?;
    for player in players {
        println!("Player: {:?}", player);
    }

    Ok(())
}
