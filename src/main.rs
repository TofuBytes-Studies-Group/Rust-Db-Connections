mod psqldb;
mod models;
mod redisdb;

use psqldb::Database;
use models::player::Player;
use redisdb::{RedisDatabase, GameScore};
use tokio_postgres::Error;
use std::io::{self, Write};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Database::connect().await?;
    let mut redis_db = RedisDatabase::connect().await.expect("Failed to connect to Redis");
    loop {
        // Print the menu
        println!("\n=== Player Management Menu ===");
        println!("1. Create a New Player");
        println!("2. List All Players");
        println!("3. Get Player by ID");
        println!("4. Update Player Health");
        println!("5. Delete Player");
        println!("6. Update Player Inventory");
        println!("7. Save Player Score");
        println!("8. Get Player Score");
        println!("9. Delete Player Score");
        println!("0. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();  // Flush to ensure the prompt shows

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                // Create a new player
                let name = prompt("Enter player name: ");
                let health = prompt("Enter player health: ").parse::<i32>().unwrap_or(100);
                let player = Player::create(&db, &name, health).await?;
                println!("Created player: {:?}", player);
            }
            "2" => {
                // List all players
                let players = Player::get_all(&db).await?;
                println!("\n=== Player List ===");
                for player in players {
                    println!("{:?}", player);
                }
            }
            "3" => {
                let id = prompt("Enter player id: ");
                match Player::get_by_id(&db, id.parse::<Uuid>().unwrap()).await? {
                    Some(player) => println!("Player found: {:?}", player),
                    None => println!("Player not found."),
                }
            }
            "4" => {
                let id = prompt("Enter player id: ");
                let health = prompt("Enter new health: ").parse::<i32>().unwrap_or(100);
                match Player::update_health(&db, id.parse::<Uuid>().unwrap(), health).await?.inventory_id {
                    Some(player) => println!("Updated player: {:?}", player),
                    None => println!("Player not found."),
                }
            }
            "5" => {
                let id_str = prompt("Enter player ID to delete: ");
                match Uuid::parse_str(&id_str) {
                    Ok(id) => {
                        let rows_deleted = Player::delete(&db, id).await?;
                        if rows_deleted > 0 {
                            println!("Player with ID {} deleted successfully.", id);
                        } else {
                            println!("No player found with ID {}.", id);
                        }
                    }
                    Err(_) => {
                        println!("Invalid UUID format.");
                    }
                }
            }
            "6" => {
                let player_id_str = prompt("Enter player ID to update inventory: ");
                match Uuid::parse_str(&player_id_str) {
                    Ok(player_id) => {
                        let inventory_id_str = prompt("Enter new inventory ID: ");
                        match Uuid::parse_str(&inventory_id_str) {
                            Ok(inv_id) => {
                                match Player::update_inventory(&db, player_id, Some(inv_id)).await {
                                    Ok(player) => println!("Updated player: {:?}", player),
                                    Err(err) => println!("Failed to update player: {}", err),
                                }
                            }
                            Err(_) => {
                                println!("Invalid inventory UUID format.");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Invalid player UUID format.");
                    }
                }
            }
            "7" => {
                let player_name = prompt("Enter player name: ");
                let kills = prompt("Enter number of kills: ").parse::<u32>().unwrap_or(0);
                let gold = prompt("Enter amount of gold: ").parse::<u32>().unwrap_or(0);
                let game_time = prompt("Enter game time (in minutes): ").parse::<u32>().unwrap_or(0);

                let score = GameScore {
                    player_name: player_name.clone(),
                    kills,
                    gold,
                    game_time,
                };

                redis_db.save_score(&player_name, score).await.expect("Failed to save score");
                println!("Score saved for player: {}", player_name);
            }
            "8" => {
                let player_name = prompt("Enter player name: ");
                match redis_db.get_score(&player_name).await {
                    Ok(Some(score)) => println!("Score for {}: {:?}", player_name, score),
                    Ok(None) => println!("No score found for player: {}", player_name),
                    Err(err) => println!("Failed to get score: {}", err),
                }
            }
            "9" => {
                let player_name = prompt("Enter player name to delete score: ");
                match redis_db.delete_score(&player_name).await {
                    Ok(_) => println!("Score deleted for player: {}", player_name),
                    Err(err) => println!("Failed to delete score: {}", err),
                }
            }
            "0" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }

    Ok(())
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
