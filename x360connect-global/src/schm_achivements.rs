use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AchievementStrings {
    pub caption: String,
    pub description: String,
    pub unachieved: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AchievementType {
    Completion = 1,
    Leveling = 2,
    Unlock = 3,
    Event = 4,
    Tournament = 5,
    Checkpoint = 6,
    Other = 7,
}

impl AchievementType {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(AchievementType::Completion),
            2 => Some(AchievementType::Leveling),
            3 => Some(AchievementType::Unlock),
            4 => Some(AchievementType::Event),
            5 => Some(AchievementType::Tournament),
            6 => Some(AchievementType::Checkpoint),
            7 => Some(AchievementType::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Achievement {
    pub id: u32,
    pub cred: u32,
    pub hidden: u32, // 0: not hidden, 1: hidden
    pub imageid: u16,
    pub strings: AchievementStrings,
    pub ach_type: AchievementType,
}