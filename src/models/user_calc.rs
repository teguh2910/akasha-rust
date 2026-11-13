//! Character calculation models for a user.

use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

use super::deserialize_u64;

/// The weapon used in a calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCalcWeapon {
    pub name: String,
    pub icon: String,
    pub substat: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub rarity: u8,
    pub refinement: u8,
}

/// A calculation variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCalcVariant {
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

/// A single character calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCalc {
    #[serde(rename = "calculationId", deserialize_with = "deserialize_u64")]
    pub id: u64,
    pub short: String,
    pub name: String,
    #[serde(default)]
    pub details: String,
    pub weapon: CharacterCalcWeapon,
    pub result: f64,
    #[serde(default)]
    pub stats: Option<HashMap<String, f64>>,
    pub ranking: i64,
    #[serde(rename = "outOf")]
    pub out_of: i64,
    #[serde(default)]
    pub variant: Option<CharacterCalcVariant>,
}

impl CharacterCalc {
    /// The percentage of the leaderboard this calculation is placed in.
    pub fn top_percent(&self) -> f64 {
        (self.ranking as f64 / self.out_of as f64) * 100.0
    }
}

/// An artifact set used by a build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCalcArtifact {
    pub name: String,
    pub icon: String,
    pub count: i64,
}

/// The calculation data of a user's character.
#[derive(Debug, Clone, Serialize)]
pub struct UserCalc {
    pub id: String,
    pub character_id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub uid: String,
    pub artifact_sets: Vec<UserCalcArtifact>,
    pub calculations: Vec<CharacterCalc>,
    pub constellation: u8,
    pub md5: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
struct UserCalcArtifactRaw {
    icon: String,
    count: i64,
}

#[derive(Debug, Deserialize)]
struct UserCalcRaw {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "characterId")]
    character_id: i64,
    #[serde(rename = "type")]
    r#type: String,
    uid: String,
    #[serde(rename = "artifactSets", default)]
    artifact_sets: Option<IndexMap<String, UserCalcArtifactRaw>>,
    #[serde(default)]
    calculations: Option<IndexMap<String, CharacterCalc>>,
    constellation: u8,
    md5: String,
    name: String,
    icon: String,
}

impl From<UserCalcRaw> for UserCalc {
    fn from(raw: UserCalcRaw) -> Self {
        let artifact_sets = raw
            .artifact_sets
            .unwrap_or_default()
            .into_iter()
            .map(|(name, artifact)| UserCalcArtifact {
                name,
                icon: artifact.icon,
                count: artifact.count,
            })
            .collect();

        let calculations = raw.calculations.unwrap_or_default().into_values().collect();

        Self {
            id: raw.id,
            character_id: raw.character_id,
            r#type: raw.r#type,
            uid: raw.uid,
            artifact_sets,
            calculations,
            constellation: raw.constellation,
            md5: raw.md5,
            name: raw.name,
            icon: raw.icon,
        }
    }
}

impl<'de> Deserialize<'de> for UserCalc {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = UserCalcRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}
