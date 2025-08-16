use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchmGame {
    pub valid_categories: Option<Vec<String>>,
    pub parsed: Option<bool>,
    pub categories: Option<Vec<Category>>,
    pub rating_descriptors: Option<Vec<RatingDescriptor>>,
    pub media_type: Option<String>,
    pub game_title_media_id: Option<String>,
    pub reduced_title: Option<String>,
    pub reduced_description: Option<String>,
    pub availability_date: Option<String>,
    pub release_date: Option<String>,
    pub rating_id: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub newest_offer_start_date: Option<String>,
    pub total_offer_count: Option<String>,
    pub total_subscription_count: Option<String>,
    pub title_id: Option<String>,
    pub effective_title_id: Option<String>,
    pub game_reduced_title: Option<String>,
    pub fulltitle: Option<String>,
    pub description: Option<String>,
    pub rating_aggregate: Option<String>,
    pub number_of_ratings: Option<String>,
    pub images: Option<Images>,
    pub video: Option<Video>,
    pub game_capabilities: Option<GameCapabilities>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub categoryid: Option<String>,
    pub system: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RatingDescriptor {
    pub ratingdescriptorid: Option<String>,
    pub ratingdescriptorlevel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub screenshots: Option<Vec<String>>,
    pub boxart: Option<BoxArt>,
    pub icon: Option<String>,
    pub background: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoxArt {
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub videodefinition: Option<String>,
    pub videoencoding: Option<String>,
    pub audioencoding: Option<String>,
    pub isacquirable: Option<String>,
    pub aspectratio: Option<String>,
    pub resolution: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameCapabilities {
    pub offlineplayersmin: Option<String>,
    pub offlineplayersmax: Option<String>,
    pub offlinecoopplayersmin: Option<String>,
    pub offlinecoopplayersmax: Option<String>,
    pub offlinesystemlinkmin: Option<String>,
    pub offlinesystemlinkmax: Option<String>,
    pub offlinemaxhdtvmodeid: Option<String>,
    pub offlinedolbydigital: Option<String>,
    pub onlinemultiplayermin: Option<String>,
    pub onlinemultiplayermax: Option<String>,
    pub onlinecoopplayersmin: Option<String>,
    pub onlinecoopplayersmax: Option<String>,
    pub onlinecontentdownload: Option<String>,
    pub onlineleaderboards: Option<String>,
    pub onlinevoice: Option<String>,
}