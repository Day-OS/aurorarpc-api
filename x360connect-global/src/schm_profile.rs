use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

// ======== AURORA STRUCTS ========

pub struct PlayersAchievements{
    #[serde(rename = "id")]
    pub _id: u8,
    pub player: Vec<u8>, // Actually thats just 0 and 1... 1 meaning it does have the achievement
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileBasic{
    pub index: u8,
    pub gamertag: String,
    pub gamerscore: u32,
    pub signedin: u8,
    pub xuid: String,
}


// ======== REQUEST STRUCTS ========

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileUnion{
    pub base: ProfileBasic,
    pub achievements: Vec<u8> // Achievement codes that this user contain
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchmProfile{
    pub current_game: String, // The title that is running at the moment of the uplod
    pub profiles: Vec<ProfileUnion>
}

impl SchmProfile{
    pub fn new(current_game_id: String, achievement_body: Vec<PlayersAchievements>, profile_body: Vec<ProfileBasic>) -> Self {
        let mut profiles: Vec<ProfileUnion> = vec![];
        for profile in profile_body {
        
            let mut achievements: Vec<u8> = vec![];
            let index = profile.index;
            for achievement in &achievement_body{
                let user_has_achvmnt: &u8 = achievement.player.get(index as usize).unwrap();
                let user_has_achvmnt = *user_has_achvmnt == 1;

                if user_has_achvmnt{
                    achievements.push(achievement._id)
                }
            }

            profiles.push(
                ProfileUnion { base: profile, achievements }
            );
        }
        SchmProfile{
            current_game: current_game_id,
            profiles
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchmProfileUploadResponse{
    pub needs_picture_update: Vec<String>, //xuid
}