use akasha::{
    Artifact, ArtifactStatType, CharaStatType, Element, Leaderboard, LeaderboardCategory,
    NameCardOrId, Profile, UserCalc,
};

#[test]
fn artifact_is_transformed() {
    let artifact: Artifact = serde_json::from_str(
        r#"{
            "name": "Gladiator's Nostalgia",
            "setName": "Gladiator's Finale",
            "stars": 5,
            "mainStatKey": "HP%",
            "mainStatValue": 0.466,
            "substats": { "Crit RATE": 0.078, "Crit DMG": 0.14 },
            "equipType": "EQUIP_BRACER",
            "level": 20,
            "critValue": 37.4,
            "icon": "flower"
        }"#,
    )
    .unwrap();

    assert_eq!(artifact.rarity, 5);
    assert_eq!(artifact.level, 19);
    assert_eq!(artifact.main_stat.stat_type, ArtifactStatType::HpPercent);
    assert_eq!(artifact.main_stat.display_value(), "46.6%");
    assert_eq!(artifact.substats.len(), 2);
    assert_eq!(artifact.substats[0].stat_type, ArtifactStatType::CritRate);
    assert_eq!(artifact.substats[0].display_value(), "7.8%");
}

#[test]
fn user_calc_flattens_sets_and_calculations() {
    let user_calc: UserCalc = serde_json::from_str(
        r#"{
            "_id": "u1",
            "characterId": 10000098,
            "type": "Nahida",
            "uid": "901211014",
            "artifactSets": {
                "Deepwood Memories": { "icon": "set1", "count": 4 }
            },
            "calculations": {
                "abc": {
                    "calculationId": "1000000203",
                    "short": "nahida",
                    "name": "Nahida",
                    "weapon": {
                        "name": "A Thousand Floating Dreams",
                        "icon": "w",
                        "substat": "Elemental Mastery",
                        "type": "Catalyst",
                        "rarity": 5,
                        "refinement": 1
                    },
                    "result": 123456.7,
                    "ranking": 5,
                    "outOf": 100
                }
            },
            "constellation": 2,
            "md5": "m",
            "name": "Nahida",
            "icon": "c"
        }"#,
    )
    .unwrap();

    assert_eq!(user_calc.artifact_sets.len(), 1);
    assert_eq!(user_calc.artifact_sets[0].name, "Deepwood Memories");
    assert_eq!(user_calc.artifact_sets[0].count, 4);
    assert_eq!(user_calc.calculations.len(), 1);
    assert_eq!(user_calc.calculations[0].id, 1000000203);
    assert_eq!(user_calc.calculations[0].top_percent(), 5.0);
    assert_eq!(user_calc.calculations[0].weapon.r#type, "Catalyst");
}

#[test]
fn leaderboard_unnests_stats_and_fields() {
    let leaderboard: Leaderboard = serde_json::from_str(
        r#"{
            "_id": "lb1",
            "calculation": { "id": "calc", "result": 123456.7 },
            "type": "Nahida",
            "uid": "123",
            "owner": {
                "nickname": "Tester",
                "adventureRank": 60.0,
                "profilePicture": { "assets": { "icon": "avatar", "oldIcon": null } },
                "nameCard": "card",
                "region": "Asia"
            },
            "index": 1,
            "md5": "m",
            "characterId": 10000098,
            "constellation": 2,
            "critValue": 200.0,
            "propMap": {
                "ascension": { "val": "6" },
                "level": { "val": "90" }
            },
            "stats": {
                "maxHp": { "value": 20000 },
                "unknownStat": { "value": 1 }
            },
            "talentsLevelMap": {
                "auto": { "icon": "i", "level": 10, "rawLevel": 10, "id": 1, "boosted": false }
            },
            "weapon": {
                "weaponInfo": {
                    "level": 90,
                    "promoteLevel": 6,
                    "refinementLevel": { "value": 1 }
                },
                "icon": "wi",
                "name": "Weapon"
            },
            "characterMetadata": { "element": "Dendro" }
        }"#,
    )
    .unwrap();

    assert_eq!(leaderboard.stats.len(), 1);
    assert_eq!(
        leaderboard.stats.get(&CharaStatType::MaxHp).unwrap().value,
        20000.0
    );
    assert_eq!(leaderboard.element, Element::Dendro);
    assert_eq!(leaderboard.ascension, 6);
    assert_eq!(leaderboard.level, 90);
    assert_eq!(leaderboard.rank, 1);
    assert_eq!(
        leaderboard.owner.profile_picture.icon.as_deref(),
        Some("https://enka.network/ui/avatar.png")
    );
    assert_eq!(
        leaderboard.owner.namecard,
        "https://enka.network/ui/card.png"
    );
    assert_eq!(leaderboard.weapon.ascension, 6);
    assert_eq!(leaderboard.weapon.refinement, 1);
}

#[test]
fn profile_converts_numbers_and_accepts_name_card_id() {
    let profile: Profile = serde_json::from_str(
        r#"{
            "index": 0,
            "_id": "p1",
            "uid": "123",
            "playerInfo": {
                "nickname": "Tester",
                "level": 60.0,
                "region": "Asia",
                "nameCardId": 210001,
                "finishAchievementNum": 800.0,
                "worldLevel": " ",
                "maxFriendshipNum": 10
            },
            "lastProfileUpdate": 1697652512985,
            "profilePictureLink": "pp",
            "nameCardLink": "nc"
        }"#,
    )
    .unwrap();

    assert_eq!(profile.player_info.level, 60);
    assert_eq!(profile.player_info.achievement_count, 800);
    assert_eq!(profile.player_info.world_level, Some(0));
    assert_eq!(profile.player_info.max_friendship_count, 10);
    assert_eq!(
        profile.updated_at.unwrap().timestamp_millis(),
        1697652512985
    );
    assert!(matches!(
        profile.player_info.name_card,
        NameCardOrId::Id(210001)
    ));
}

#[test]
fn category_filters_empty_teammates() {
    let category: LeaderboardCategory = serde_json::from_str(
        r#"{
            "_id": "c1",
            "addDate": "2024-01-01T00:00:00Z",
            "characterId": 10000098,
            "characterName": "Nahida",
            "weapons": [
                {
                    "name": "Weapon",
                    "icon": "i",
                    "substat": "Elemental Mastery",
                    "rarity": 5,
                    "refinement": 1,
                    "calculationId": "calc",
                    "details": "d",
                    "short": "w",
                    "teammates": [
                        { "foo": 1 },
                        {
                            "name": "Nahida",
                            "character": {
                                "name": "Nahida",
                                "element": "Dendro",
                                "rarity": 5,
                                "icon": "i",
                                "constellation": 2
                            },
                            "weapon": null
                        }
                    ],
                    "filters": [
                        { "name": "fid", "displayName": "Filter" }
                    ]
                }
            ],
            "count": 1,
            "details": "d",
            "element": "Dendro",
            "name": "Nahida",
            "rarity": 5,
            "short": "n",
            "characterIcon": "ci",
            "index": 0
        }"#,
    )
    .unwrap();

    assert_eq!(category.weapons.len(), 1);
    assert_eq!(category.weapons[0].teammates.len(), 1);
    assert_eq!(category.weapons[0].filters.len(), 1);
    assert_eq!(category.weapons[0].filters[0].id, "fid");
}
