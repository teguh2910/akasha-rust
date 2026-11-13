//! Leaderboard category models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::deserialize_datetime;

/// A character appearing in a leaderboard's team composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeammateCharacter {
    pub name: String,
    pub element: String,
    pub rarity: u8,
    pub icon: String,
    pub constellation: Option<u8>,
}

/// A weapon appearing in a leaderboard's team composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeammateWeapon {
    pub name: String,
    pub rarity: u8,
    pub icon: String,
    pub refinement: u8,
}

/// A teammate entry of a leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardTeammate {
    pub character: TeammateCharacter,
    #[serde(default)]
    pub weapon: Option<TeammateWeapon>,
}

/// A filter selectable on a weapon leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponLeaderboardFilter {
    #[serde(rename = "name")]
    pub id: String,
    #[serde(rename = "displayName")]
    pub name: String,
}

/// A weapon leaderboard of a character.
#[derive(Debug, Clone, Serialize)]
pub struct WeaponLeaderboard {
    pub name: String,
    pub icon: String,
    pub substat: String,
    pub rarity: u8,
    pub refinement: u8,
    pub calculation_id: String,
    pub details: String,
    pub short_name: String,
    pub teammates: Vec<LeaderboardTeammate>,
    pub filters: Vec<WeaponLeaderboardFilter>,
}

#[derive(Debug, Deserialize)]
struct WeaponLeaderboardRaw {
    name: String,
    icon: String,
    substat: String,
    rarity: u8,
    refinement: u8,
    #[serde(rename = "calculationId")]
    calculation_id: String,
    details: String,
    #[serde(rename = "short")]
    short_name: String,
    #[serde(default)]
    teammates: Vec<Value>,
    #[serde(default)]
    filters: Option<Vec<WeaponLeaderboardFilter>>,
}

impl<'de> Deserialize<'de> for WeaponLeaderboard {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = WeaponLeaderboardRaw::deserialize(deserializer)?;

        let teammates = raw
            .teammates
            .into_iter()
            .filter(|teammate| teammate.get("name").is_some())
            .map(|teammate| serde_json::from_value(teammate).map_err(serde::de::Error::custom))
            .collect::<std::result::Result<Vec<LeaderboardTeammate>, D::Error>>()?;

        Ok(Self {
            name: raw.name,
            icon: raw.icon,
            substat: raw.substat,
            rarity: raw.rarity,
            refinement: raw.refinement,
            calculation_id: raw.calculation_id,
            details: raw.details,
            short_name: raw.short_name,
            teammates,
            filters: raw.filters.unwrap_or_default(),
        })
    }
}

/// A character's leaderboard category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardCategory {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "addDate", deserialize_with = "deserialize_datetime")]
    pub added_date: DateTime<Utc>,
    #[serde(rename = "characterId")]
    pub character_id: i64,
    #[serde(rename = "characterName")]
    pub character_name: String,
    pub weapons: Vec<WeaponLeaderboard>,
    pub count: i64,
    pub details: String,
    pub element: String,
    pub name: String,
    pub rarity: i64,
    #[serde(rename = "short")]
    pub short_name: String,
    #[serde(rename = "characterIcon")]
    pub character_icon: String,
    pub index: i64,
}
