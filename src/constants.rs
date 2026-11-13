//! Stat constants shared by the models.

use crate::enums::{ArtifactStat, CharaStatType};

/// Character stats that are displayed as a percentage.
pub const PERCENT_CHARA_STATS: &[CharaStatType] = &[
    CharaStatType::CritRate,
    CharaStatType::CritDmg,
    CharaStatType::CryoDmgBonus,
    CharaStatType::PyroDmgBonus,
    CharaStatType::HydroDmgBonus,
    CharaStatType::AnemoDmgBonus,
    CharaStatType::ElectroDmgBonus,
    CharaStatType::GeoDmgBonus,
    CharaStatType::DendroDmgBonus,
    CharaStatType::HealingBonus,
    CharaStatType::EnergyRecharge,
];

/// Artifact stats that are displayed as a percentage.
pub const PERCENT_ARTIFACT_STATS: &[ArtifactStat] = &[
    ArtifactStat::EnergyRecharge,
    ArtifactStat::CritRate,
    ArtifactStat::CritDmg,
    ArtifactStat::HpPercent,
    ArtifactStat::AtkPercent,
    ArtifactStat::DefPercent,
    ArtifactStat::CryoDmgBonus,
    ArtifactStat::PyroDmgBonus,
    ArtifactStat::HydroDmgBonus,
    ArtifactStat::AnemoDmgBonus,
    ArtifactStat::ElectroDmgBonus,
    ArtifactStat::GeoDmgBonus,
    ArtifactStat::DendroDmgBonus,
];

pub fn is_percent_chara_stat(stat: CharaStatType) -> bool {
    PERCENT_CHARA_STATS.contains(&stat)
}

pub fn is_percent_artifact_stat(stat: ArtifactStat) -> bool {
    PERCENT_ARTIFACT_STATS.contains(&stat)
}
