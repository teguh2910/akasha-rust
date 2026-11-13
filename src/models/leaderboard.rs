//! Leaderboard models.

use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use tracing::warn;

use crate::constants::is_percent_chara_stat;
use crate::enums::{CharaStatType, Element};

use super::profile::PlayerInfo;
use super::{deserialize_int, icon_url, parse_enum, value_to_i64};

/// The calculation result attached to a leaderboard entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardCalc {
    pub id: String,
    pub result: f64,
}

/// A player's profile picture.
#[derive(Debug, Clone, Serialize)]
pub struct ProfilePicture {
    pub icon: Option<String>,
    pub old_icon: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfilePictureAssetsRaw {
    #[serde(default)]
    icon: Option<String>,
    #[serde(rename = "oldIcon", default)]
    old_icon: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfilePictureRaw {
    #[serde(default)]
    icon: Option<String>,
    #[serde(rename = "oldIcon", default)]
    old_icon: Option<String>,
    #[serde(default)]
    assets: Option<ProfilePictureAssetsRaw>,
}

impl From<ProfilePictureRaw> for ProfilePicture {
    fn from(raw: ProfilePictureRaw) -> Self {
        // `assets` takes precedence over the top level fields.
        let (icon, old_icon) = match raw.assets {
            Some(assets) => (assets.icon.or(raw.icon), assets.old_icon.or(raw.old_icon)),
            None => (raw.icon, raw.old_icon),
        };

        Self {
            icon: icon.as_deref().map(icon_url),
            old_icon: old_icon.as_deref().map(icon_url),
        }
    }
}

impl<'de> Deserialize<'de> for ProfilePicture {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = ProfilePictureRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// The owner of a leaderboard entry.
#[derive(Debug, Clone, Serialize)]
pub struct LeaderboardOwner {
    pub nickname: String,
    pub adventure_rank: i64,
    pub profile_picture: ProfilePicture,
    pub namecard: String,
    pub region: String,
}

#[derive(Debug, Deserialize)]
struct LeaderboardOwnerRaw {
    nickname: String,
    #[serde(rename = "adventureRank", deserialize_with = "deserialize_int")]
    adventure_rank: i64,
    #[serde(rename = "profilePicture")]
    profile_picture: ProfilePicture,
    #[serde(rename = "nameCard")]
    namecard: String,
    region: String,
}

impl From<LeaderboardOwnerRaw> for LeaderboardOwner {
    fn from(raw: LeaderboardOwnerRaw) -> Self {
        Self {
            nickname: raw.nickname,
            adventure_rank: raw.adventure_rank,
            profile_picture: raw.profile_picture,
            namecard: icon_url(&raw.namecard),
            region: raw.region,
        }
    }
}

impl<'de> Deserialize<'de> for LeaderboardOwner {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = LeaderboardOwnerRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// A character talent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTalent {
    pub icon: String,
    pub level: i64,
    #[serde(rename = "rawLevel")]
    pub raw_level: i64,
    pub id: i64,
    pub boosted: bool,
}

/// A character's equipped weapon.
#[derive(Debug, Clone, Serialize)]
pub struct CharacterWeapon {
    pub level: i64,
    pub ascension: i64,
    pub refinement: i64,
    pub icon: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct RefinementLevelRaw {
    value: i64,
}

#[derive(Debug, Deserialize)]
struct WeaponInfoRaw {
    level: i64,
    #[serde(rename = "promoteLevel", default)]
    promote_level: Option<i64>,
    #[serde(rename = "refinementLevel")]
    refinement_level: RefinementLevelRaw,
}

#[derive(Debug, Deserialize)]
struct CharacterWeaponRaw {
    #[serde(rename = "weaponInfo")]
    weapon_info: WeaponInfoRaw,
    icon: String,
    name: String,
}

impl From<CharacterWeaponRaw> for CharacterWeapon {
    fn from(raw: CharacterWeaponRaw) -> Self {
        Self {
            level: raw.weapon_info.level,
            ascension: raw.weapon_info.promote_level.unwrap_or(0),
            refinement: raw.weapon_info.refinement_level.value,
            icon: raw.icon,
            name: raw.name,
        }
    }
}

impl<'de> Deserialize<'de> for CharacterWeapon {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = CharacterWeaponRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// A character stat with a human readable display value.
#[derive(Debug, Clone, Serialize)]
pub struct CharacterStat {
    pub stat_type: CharaStatType,
    pub value: f64,
}

impl CharacterStat {
    pub fn display_value(&self) -> String {
        if is_percent_chara_stat(self.stat_type) {
            format!("{:.1}%", self.value * 100.0)
        } else {
            format!("{}", self.value.round())
        }
    }
}

#[derive(Debug, Deserialize)]
struct StatValueRaw {
    value: f64,
}

fn deserialize_stats<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<CharaStatType, CharacterStat>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = IndexMap::<String, StatValueRaw>::deserialize(deserializer)?;
    let mut stats = HashMap::with_capacity(raw.len());

    for (name, value) in raw {
        match parse_enum::<CharaStatType>(&name) {
            Some(stat_type) => {
                stats.insert(
                    stat_type,
                    CharacterStat {
                        stat_type,
                        value: value.value,
                    },
                );
            }
            None => warn!("Unknown stat type: {}, skipping.", name),
        }
    }

    Ok(stats)
}

#[derive(Debug, Deserialize)]
struct PropValueRaw {
    #[serde(default)]
    val: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct PropMapRaw {
    ascension: PropValueRaw,
    level: PropValueRaw,
}

#[derive(Debug, Deserialize)]
struct CharacterMetadataRaw {
    element: Element,
}

/// A character leaderboard entry.
#[derive(Debug, Clone, Serialize)]
pub struct Leaderboard {
    pub id: String,
    pub calculation: LeaderboardCalc,
    pub build_name: String,
    pub uid: String,
    pub owner: LeaderboardOwner,
    pub rank: i64,
    pub md5: String,

    pub character_id: i64,
    pub constellation: i64,
    pub crit_value: f64,
    pub ascension: i64,
    pub level: i64,
    pub stats: HashMap<CharaStatType, CharacterStat>,
    pub talents: HashMap<String, CharacterTalent>,
    pub weapon: CharacterWeapon,
    pub element: Element,
}

#[derive(Debug, Deserialize)]
struct LeaderboardRaw {
    #[serde(rename = "_id")]
    id: String,
    calculation: LeaderboardCalc,
    #[serde(rename = "type")]
    build_name: String,
    uid: String,
    owner: LeaderboardOwner,
    #[serde(rename = "index")]
    rank: i64,
    md5: String,
    #[serde(rename = "characterId")]
    character_id: i64,
    constellation: i64,
    #[serde(rename = "critValue")]
    crit_value: f64,
    #[serde(rename = "propMap")]
    prop_map: PropMapRaw,
    #[serde(deserialize_with = "deserialize_stats")]
    stats: HashMap<CharaStatType, CharacterStat>,
    #[serde(rename = "talentsLevelMap")]
    talents: HashMap<String, CharacterTalent>,
    weapon: CharacterWeapon,
    #[serde(rename = "characterMetadata")]
    character_metadata: CharacterMetadataRaw,
}

impl From<LeaderboardRaw> for Leaderboard {
    fn from(raw: LeaderboardRaw) -> Self {
        let ascension = raw
            .prop_map
            .ascension
            .val
            .as_ref()
            .and_then(value_to_i64)
            .unwrap_or(0);

        let level = match raw.prop_map.level.val.as_ref().and_then(value_to_i64) {
            Some(level) if level != 0 => level,
            _ => 1,
        };

        Self {
            id: raw.id,
            calculation: raw.calculation,
            build_name: raw.build_name,
            uid: raw.uid,
            owner: raw.owner,
            rank: raw.rank,
            md5: raw.md5,
            character_id: raw.character_id,
            constellation: raw.constellation,
            crit_value: raw.crit_value,
            ascension,
            level,
            stats: raw.stats,
            talents: raw.talents,
            weapon: raw.weapon,
            element: raw.character_metadata.element,
        }
    }
}

impl<'de> Deserialize<'de> for Leaderboard {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = LeaderboardRaw::deserialize(deserializer)?;
        Ok(raw.into())
    }
}

/// A Stygian Onslaught leaderboard entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StygianLeaderboard {
    #[serde(rename = "_id")]
    pub id: String,
    pub uid: String,
    #[serde(rename = "isCheating")]
    pub is_cheating: bool,
    pub owner: LeaderboardOwner,
    #[serde(rename = "playerInfo")]
    pub player_info: PlayerInfo,
    #[serde(rename = "index")]
    pub rank: i64,

    #[serde(rename = "stygianIndex")]
    pub stygian_index: i64,
    #[serde(rename = "stygianScore")]
    pub stygian_score: f64,
    #[serde(rename = "stygianSeconds")]
    pub stygian_seconds: i64,
    #[serde(rename = "stygianId", default)]
    pub stygian_id: Option<i64>,

    #[serde(rename = "susLevel")]
    pub sus_level: i64,
    #[serde(rename = "nameCardLink")]
    pub name_card_url: String,
    #[serde(rename = "profilePictureLink")]
    pub profile_picture_url: String,
}
