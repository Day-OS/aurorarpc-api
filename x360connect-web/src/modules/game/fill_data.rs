use std::path::Path;

use ordermap::OrderSet;
use rocket::tokio;
use x360connect_global::schm_game::SchmGame;

use crate::modules::game::model::Game;

pub async fn fill_data(json_path: &Path, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
    let json = tokio::fs::read_to_string(json_path).await?;

    let games: Vec<SchmGame> = serde_json::from_str(&json)?;

    for game in games {
        println!("Filling game database with title \"{}\" - ID:{}", game.fulltitle, game.title_id);
        
        Game{ 
            id: None,
            game_id: game.title_id, 
            schema: game, 
            images_were_downloaded: false,
            achievements_were_downloaded: false,
            achievements: OrderSet::new(),
        }.new(&db).await?;
    }

    

    Ok(())
}