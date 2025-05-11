use mongodb::{bson::doc, Client, Collection, Database, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::error::Error;
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub character_name: String,
    pub world_lore: String,

}

pub struct MongoDB {
    db: Database,
    collection: Collection<Item>,
}

impl MongoDB {
    pub async fn new(connection_string: &str, db_name: &str, collection_name: &str) -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);
        let collection = db.collection::<Item>(collection_name);
        Ok(Self { db, collection })
    }

    pub async fn create(&self, mut item: Item) -> Result<ObjectId, Box<dyn Error>> {
        let result = self.collection.insert_one(&item, None).await?;
        if let Some(id) = result.inserted_id.as_object_id() {
            Ok(id)
        } else {
            Err("Failed to get the inserted ID".into())
        }
    }

    pub async fn read(&self, item_id: &str) -> Result<Option<Item>, Box<dyn Error>> {
        let object_id = ObjectId::parse_str(item_id)?;
        let filter = doc! { "_id": object_id };
        let item = self.collection.find_one(filter, None).await?;
        Ok(item)
    }


    pub async fn update(&self, item_id: &str, new_world_lore: &str) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectId::parse_str(item_id)?;
        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": { "world_lore": new_world_lore } };
        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete(&self, item_id: &str) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectId::parse_str(item_id)?;
        let filter = doc! { "_id": object_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }

    // pub async fn list_all(&self) -> Result<Vec<Item>, Box<dyn Error>> {
    //     let cursor = self.collection.find(None, None).await?;
    //     let items: Vec<Item> = cursor.try_collect().await?;
    //     Ok(items)
    // }
}
