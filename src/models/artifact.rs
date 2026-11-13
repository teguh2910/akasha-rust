//! Artifact models.

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};

use crate::constants::is_percent_artifact_stat;
use crate::enums::{ArtifactStat as ArtifactStatType, EquipType};
use crate::errors::{Error, Result};

use super::parse_enum;

/// A single artifact stat, either the main stat or a substat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactStat {
    #[serde(rename = "name")]
    pub stat_type: ArtifactStatType,
    pub value: f64,
}

impl ArtifactStat {
    /// Human readable value, e.g. `"46.6%"` or `"4780"`.
    pub fn display_value(&self) -> String {
        if is_percent_artifact_stat(self.stat_type) {
            format!("{:.1}%", self.value * 100.0)
        } else {
            format!("{}", self.value.round())
        }
    }
}

/// An equipped artifact.
#[derive(Debug, Clone, Serialize)]
pub struct Artifact {
    pub name: String,
    pub set_name: String,
    pub rarity: u8,
    pub main_stat: ArtifactStat,
    pub substats: Vec<ArtifactStat>,
    pub equip_type: EquipType,
    pub level: i64,
    pub crit_value: f64,
    pub icon: String,
}

#[derive(Debug, Deserialize)]
struct ArtifactRaw {
    name: String,
    #[serde(rename = "setName")]
    set_name: String,
    #[serde(rename = "stars")]
    rarity: u8,
    #[serde(rename = "mainStatKey")]
    main_stat_key: ArtifactStatType,
    #[serde(rename = "mainStatValue")]
    main_stat_value: f64,
    #[serde(default)]
    substats: IndexMap<String, f64>,
    #[serde(rename = "equipType")]
    equip_type: EquipType,
    level: i64,
    #[serde(rename = "critValue")]
    crit_value: f64,
    icon: String,
}

impl TryFrom<ArtifactRaw> for Artifact {
    type Error = Error;

    fn try_from(raw: ArtifactRaw) -> Result<Self> {
        let substats = raw
            .substats
            .into_iter()
            .map(|(name, value)| {
                let stat_type =
                    parse_enum(&name).ok_or_else(|| Error::UnknownVariant(name.clone()))?;
                Ok(ArtifactStat { stat_type, value })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            name: raw.name,
            set_name: raw.set_name,
            rarity: raw.rarity,
            main_stat: ArtifactStat {
                stat_type: raw.main_stat_key,
                value: raw.main_stat_value,
            },
            substats,
            equip_type: raw.equip_type,
            level: raw.level - 1,
            crit_value: raw.crit_value,
            icon: raw.icon,
        })
    }
}

impl<'de> Deserialize<'de> for Artifact {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = ArtifactRaw::deserialize(deserializer)?;
        Artifact::try_from(raw).map_err(serde::de::Error::custom)
    }
}
