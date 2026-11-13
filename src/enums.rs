//! Enumerations used throughout the crate.

use serde::{Deserialize, Serialize};

/// Languages supported by Akasha System.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[default]
    #[serde(rename = "en")]
    English,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "zh-CN")]
    ChineseSimplified,
    #[serde(rename = "zh-TW")]
    ChineseTraditional,
}

impl Language {
    /// Every language supported by Akasha System.
    pub const ALL: [Self; 14] = [
        Self::English,
        Self::German,
        Self::French,
        Self::Spanish,
        Self::Italian,
        Self::Japanese,
        Self::Korean,
        Self::Portuguese,
        Self::Russian,
        Self::Thai,
        Self::Turkish,
        Self::Vietnamese,
        Self::ChineseSimplified,
        Self::ChineseTraditional,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::German => "de",
            Self::French => "fr",
            Self::Spanish => "es",
            Self::Italian => "it",
            Self::Japanese => "ja",
            Self::Korean => "ko",
            Self::Portuguese => "pt",
            Self::Russian => "ru",
            Self::Thai => "th",
            Self::Turkish => "tr",
            Self::Vietnamese => "vi",
            Self::ChineseSimplified => "zh-CN",
            Self::ChineseTraditional => "zh-TW",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Artifact slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipType {
    #[serde(rename = "EQUIP_BRACER")]
    Flower,
    #[serde(rename = "EQUIP_NECKLACE")]
    Feather,
    #[serde(rename = "EQUIP_SHOES")]
    Sands,
    #[serde(rename = "EQUIP_RING")]
    Goblet,
    #[serde(rename = "EQUIP_DRESS")]
    Circlet,
}

/// Elements in Genshin Impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    #[serde(rename = "Hydro")]
    Hydro,
    #[serde(rename = "Anemo")]
    Anemo,
    #[serde(rename = "Pyro")]
    Pyro,
    #[serde(rename = "Cryo")]
    Cryo,
    #[serde(rename = "Electro")]
    Electro,
    #[serde(rename = "Geo")]
    Geo,
    #[serde(rename = "Dendro")]
    Dendro,
}

impl Element {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hydro => "Hydro",
            Self::Anemo => "Anemo",
            Self::Pyro => "Pyro",
            Self::Cryo => "Cryo",
            Self::Electro => "Electro",
            Self::Geo => "Geo",
            Self::Dendro => "Dendro",
        }
    }
}

/// Artifact stat names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactStat {
    #[serde(rename = "Crit DMG")]
    CritDmg,
    #[serde(rename = "Crit RATE")]
    CritRate,

    #[serde(rename = "Flat HP")]
    FlatHp,
    #[serde(rename = "HP%")]
    HpPercent,
    #[serde(rename = "Flat ATK")]
    FlatAtk,
    #[serde(rename = "ATK%")]
    AtkPercent,
    #[serde(rename = "Flat DEF")]
    FlatDef,
    #[serde(rename = "DEF%")]
    DefPercent,

    #[serde(rename = "Energy Recharge")]
    EnergyRecharge,
    #[serde(rename = "Elemental Mastery")]
    ElementalMastery,

    #[serde(rename = "Hydro DMG Bonus")]
    HydroDmgBonus,
    #[serde(rename = "Anemo DMG Bonus")]
    AnemoDmgBonus,
    #[serde(rename = "Pyro DMG Bonus")]
    PyroDmgBonus,
    #[serde(rename = "Cryo DMG Bonus")]
    CryoDmgBonus,
    #[serde(rename = "Electro DMG Bonus")]
    ElectroDmgBonus,
    #[serde(rename = "Geo DMG Bonus")]
    GeoDmgBonus,
    #[serde(rename = "Dendro DMG Bonus")]
    DendroDmgBonus,
}

impl ArtifactStat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CritDmg => "Crit DMG",
            Self::CritRate => "Crit RATE",
            Self::FlatHp => "Flat HP",
            Self::HpPercent => "HP%",
            Self::FlatAtk => "Flat ATK",
            Self::AtkPercent => "ATK%",
            Self::FlatDef => "Flat DEF",
            Self::DefPercent => "DEF%",
            Self::EnergyRecharge => "Energy Recharge",
            Self::ElementalMastery => "Elemental Mastery",
            Self::HydroDmgBonus => "Hydro DMG Bonus",
            Self::AnemoDmgBonus => "Anemo DMG Bonus",
            Self::PyroDmgBonus => "Pyro DMG Bonus",
            Self::CryoDmgBonus => "Cryo DMG Bonus",
            Self::ElectroDmgBonus => "Electro DMG Bonus",
            Self::GeoDmgBonus => "Geo DMG Bonus",
            Self::DendroDmgBonus => "Dendro DMG Bonus",
        }
    }
}

/// Character stat types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharaStatType {
    #[serde(rename = "maxHp")]
    MaxHp,
    #[serde(rename = "baseAtk")]
    BaseAtk,
    #[serde(rename = "atk")]
    Atk,
    #[serde(rename = "def")]
    Def,
    #[serde(rename = "elementalMastery")]
    ElementalMastery,
    #[serde(rename = "energyRecharge")]
    EnergyRecharge,
    #[serde(rename = "healingBonus")]
    HealingBonus,
    #[serde(rename = "critRate")]
    CritRate,
    #[serde(rename = "critDamage")]
    CritDmg,
    #[serde(rename = "cryoDamageBonus")]
    CryoDmgBonus,
    #[serde(rename = "pyroDamageBonus")]
    PyroDmgBonus,
    #[serde(rename = "hydroDamageBonus")]
    HydroDmgBonus,
    #[serde(rename = "anemoDamageBonus")]
    AnemoDmgBonus,
    #[serde(rename = "electroDamageBonus")]
    ElectroDmgBonus,
    #[serde(rename = "geoDamageBonus")]
    GeoDmgBonus,
    #[serde(rename = "dendroDamageBonus")]
    DendroDmgBonus,
    #[serde(rename = "physicalDamageBonus")]
    PhysicalDmgBonus,
}

impl CharaStatType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MaxHp => "maxHp",
            Self::BaseAtk => "baseAtk",
            Self::Atk => "atk",
            Self::Def => "def",
            Self::ElementalMastery => "elementalMastery",
            Self::EnergyRecharge => "energyRecharge",
            Self::HealingBonus => "healingBonus",
            Self::CritRate => "critRate",
            Self::CritDmg => "critDamage",
            Self::CryoDmgBonus => "cryoDamageBonus",
            Self::PyroDmgBonus => "pyroDamageBonus",
            Self::HydroDmgBonus => "hydroDamageBonus",
            Self::AnemoDmgBonus => "anemoDamageBonus",
            Self::ElectroDmgBonus => "electroDamageBonus",
            Self::GeoDmgBonus => "geoDamageBonus",
            Self::DendroDmgBonus => "dendroDamageBonus",
            Self::PhysicalDmgBonus => "physicalDamageBonus",
        }
    }
}

/// Fields that profiles can be sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfileSortBy {
    #[serde(rename = "playerInfo.stygianScore")]
    Stygian,
    #[serde(rename = "playerInfo.finishAchievementNum")]
    AchievementNum,
    #[serde(rename = "lastProfileUpdate")]
    UpdatedAt,
}

impl ProfileSortBy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stygian => "playerInfo.stygianScore",
            Self::AchievementNum => "playerInfo.finishAchievementNum",
            Self::UpdatedAt => "lastProfileUpdate",
        }
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OrderBy {
    Ascending = 1,
    #[default]
    Descending = -1,
}

impl OrderBy {
    pub fn value(self) -> i32 {
        self as i32
    }
}
