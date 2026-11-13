//! Player profile models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::{
    deserialize_int, deserialize_opt_datetime, deserialize_opt_int,
    deserialize_opt_int_invalid_as_zero, icon_url,
};

/// The images associated with a name card.
#[derive(Debug, Clone, Serialize)]
pub struct NameCardAsset {
    pub icon: String,
    pub pictures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct NameCardAssetRaw {
    icon: String,
    #[serde(rename = "picPath")]
    pictures: Vec<String>,
}

impl From<NameCardAssetRaw> for NameCardAsset {
    fn from(raw: NameCardAssetRaw) -> Self {
        Self {
            icon: icon_url(&raw.icon),
            pictures: raw
                .pictures
                .iter()
                .map(|picture| icon_url(picture))
                .collect(),
        }
    }
}

impl<'de> Deserialize<'de> for NameCardAsset {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = NameCardAssetRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// A player's name card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameCard {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_int")]
    pub id: i64,
    #[serde(rename = "assets")]
    pub asset: NameCardAsset,
}

/// A name card reference that may either be a full object or just an id.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NameCardOrId {
    Card(NameCard),
    Id(i64),
}

impl<'de> Deserialize<'de> for NameCardOrId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Object(_) => serde_json::from_value(value)
                .map(Self::Card)
                .map_err(serde::de::Error::custom),
            Value::Number(_) | Value::String(_) => super::value_to_i64(&value)
                .map(Self::Id)
                .ok_or_else(|| serde::de::Error::custom("expected a name card id")),
            _ => Err(serde::de::Error::custom("expected a name card or id")),
        }
    }
}

/// In-game player information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "PlayerInfoRaw")]
pub struct PlayerInfo {
    pub nickname: String,
    pub level: i64,
    pub signature: Option<String>,
    pub region: String,
    pub name_card: NameCardOrId,
    pub world_level: Option<i64>,
    /// Number of achievements completed.
    pub achievement_count: i64,
    pub abyss_floor: Option<i64>,
    pub abyss_level: Option<i64>,
    pub stygian_index: Option<i64>,
    pub stygian_seconds: Option<i64>,
    pub stygian_score: Option<f64>,
    /// Number of max friendship characters this player has.
    pub max_friendship_count: i64,
}

#[derive(Debug, Deserialize)]
struct PlayerInfoRaw {
    nickname: String,
    #[serde(deserialize_with = "deserialize_int")]
    level: i64,
    #[serde(default)]
    signature: Option<String>,
    region: String,
    #[serde(rename = "nameCardId")]
    name_card: NameCardOrId,
    #[serde(
        rename = "worldLevel",
        default,
        deserialize_with = "deserialize_opt_int_invalid_as_zero"
    )]
    world_level: Option<i64>,
    #[serde(rename = "finishAchievementNum", deserialize_with = "deserialize_int")]
    achievement_count: i64,
    #[serde(
        rename = "towerFloorIndex",
        default,
        deserialize_with = "deserialize_opt_int_invalid_as_zero"
    )]
    abyss_floor: Option<i64>,
    #[serde(
        rename = "towerLevelIndex",
        default,
        deserialize_with = "deserialize_opt_int_invalid_as_zero"
    )]
    abyss_level: Option<i64>,
    #[serde(
        rename = "stygianIndex",
        default,
        deserialize_with = "deserialize_opt_int"
    )]
    stygian_index: Option<i64>,
    #[serde(
        rename = "stygianSeconds",
        default,
        deserialize_with = "deserialize_opt_int"
    )]
    stygian_seconds: Option<i64>,
    #[serde(rename = "stygianScore", default)]
    stygian_score: Option<f64>,
    #[serde(
        rename = "maxFriendshipNum",
        default,
        deserialize_with = "deserialize_int"
    )]
    max_friendship_count: i64,
}

impl From<PlayerInfoRaw> for PlayerInfo {
    fn from(raw: PlayerInfoRaw) -> Self {
        Self {
            nickname: raw.nickname,
            level: raw.level,
            signature: raw.signature,
            region: raw.region,
            name_card: raw.name_card,
            world_level: raw.world_level,
            achievement_count: raw.achievement_count,
            abyss_floor: raw.abyss_floor,
            abyss_level: raw.abyss_level,
            stygian_index: raw.stygian_index,
            stygian_seconds: raw.stygian_seconds,
            stygian_score: raw.stygian_score,
            max_friendship_count: raw.max_friendship_count,
        }
    }
}

/// A player profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub index: i64,
    #[serde(rename = "_id")]
    pub id: String,
    pub uid: String,
    #[serde(rename = "playerInfo")]
    pub player_info: PlayerInfo,
    #[serde(rename = "ownedCharacters", default)]
    pub owned_character_ids: Vec<i64>,
    #[serde(
        rename = "lastProfileUpdate",
        default,
        deserialize_with = "deserialize_opt_datetime"
    )]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "profilePictureLink")]
    pub profile_picture_url: String,
    #[serde(rename = "nameCardLink")]
    pub name_card_url: String,
}
