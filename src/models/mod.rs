//! Data models returned by the Akasha API.

use chrono::{DateTime, TimeZone, Utc};

pub mod artifact;
pub mod category;
pub mod leaderboard;
pub mod profile;
pub mod user_calc;

pub use artifact::{Artifact, ArtifactStat};
pub use category::{
    LeaderboardCategory, LeaderboardTeammate, TeammateCharacter, TeammateWeapon, WeaponLeaderboard,
    WeaponLeaderboardFilter,
};
pub use leaderboard::{
    CharacterStat, CharacterTalent, CharacterWeapon, Leaderboard, LeaderboardCalc,
    LeaderboardOwner, ProfilePicture, StygianLeaderboard,
};
pub use profile::{NameCard, NameCardAsset, NameCardOrId, PlayerInfo, Profile};
pub use user_calc::{
    CharacterCalc, CharacterCalcVariant, CharacterCalcWeapon, UserCalc, UserCalcArtifact,
};

pub(crate) fn icon_url(icon: &str) -> String {
    format!("https://enka.network/ui/{icon}.png")
}

pub(crate) fn parse_enum<T: serde::de::DeserializeOwned>(value: &str) -> Option<T> {
    serde_json::from_value(serde_json::Value::String(value.to_owned())).ok()
}

/// Convert a JSON value into an integer the way Python's `int()` would.
pub(crate) fn value_to_i64(value: &serde_json::Value) -> Option<i64> {
    match value {
        serde_json::Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_f64().map(|float| float as i64)),
        serde_json::Value::String(string) => string.parse().ok(),
        serde_json::Value::Bool(boolean) => Some(i64::from(*boolean)),
        _ => None,
    }
}

fn value_to_u64(value: &serde_json::Value) -> Option<u64> {
    value_to_i64(value).and_then(|number| u64::try_from(number).ok())
}

fn timestamp_to_datetime(timestamp: i64) -> Option<DateTime<Utc>> {
    // Values that look like milliseconds are treated as such, otherwise as seconds.
    if timestamp.abs() >= 100_000_000_000 {
        Utc.timestamp_millis_opt(timestamp).single()
    } else {
        Utc.timestamp_opt(timestamp, 0).single()
    }
}

fn value_to_datetime(value: &serde_json::Value) -> Option<DateTime<Utc>> {
    match value {
        serde_json::Value::String(string) => DateTime::parse_from_rfc3339(string)
            .ok()
            .map(|datetime| datetime.with_timezone(&Utc))
            .or_else(|| string.parse::<i64>().ok().and_then(timestamp_to_datetime)),
        serde_json::Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_f64().map(|float| float as i64))
            .and_then(timestamp_to_datetime),
        _ => None,
    }
}

pub(crate) fn deserialize_datetime<'de, D>(
    deserializer: D,
) -> std::result::Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = serde_json::Value::deserialize(deserializer)?;
    value_to_datetime(&value)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid datetime: {value}")))
}

pub(crate) fn deserialize_opt_datetime<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.as_ref().and_then(value_to_datetime))
}

pub(crate) fn deserialize_u64<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = serde_json::Value::deserialize(deserializer)?;
    value_to_u64(&value).ok_or_else(|| {
        serde::de::Error::custom(format!("expected an unsigned integer, got {value}"))
    })
}

pub(crate) fn deserialize_int<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = serde_json::Value::deserialize(deserializer)?;
    value_to_i64(&value)
        .ok_or_else(|| serde::de::Error::custom(format!("expected an integer, got {value}")))
}

pub(crate) fn deserialize_opt_int_invalid_as_zero<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(_)) => Some(0),
        Some(value) => Some(value_to_i64(&value).unwrap_or(0)),
    })
}

pub(crate) fn deserialize_opt_int<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.as_ref().and_then(value_to_i64))
}
