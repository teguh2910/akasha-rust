//! Integration tests that hit the live akasha.cv API.
//!
//! Run them with `cargo test --test live -- --ignored`.

use akasha::{
    Akasha, Language, LeaderboardOptions, OrderBy, ProfileOptions, ProfileSortBy, StygianOptions,
};

fn api() -> Akasha {
    Akasha::new(Language::English).unwrap()
}

#[tokio::test]
#[ignore = "requires network access"]
async fn get_artifacts() {
    api()
        .get_artifacts(820863575, "69313a2a03491ee368aa75837d1da7af")
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires network access"]
async fn get_categories() {
    api().get_categories("10000098").await.unwrap();
}

#[tokio::test]
#[ignore = "requires network access"]
async fn leaderboards() {
    let api = api();
    let mut boards = api.get_leaderboards(1000000203, LeaderboardOptions::new().max_page(1));
    while let Some(board) = boards.next().await.unwrap() {
        assert!(!board.uid.is_empty());
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn leaderboards_with_uids() {
    let api = api();
    let mut boards = api.get_leaderboards(
        1000000203,
        LeaderboardOptions::new()
            .max_page(1)
            .uids([626792307, 730208419, 806331997]),
    );
    while let Some(board) = boards.next().await.unwrap() {
        assert!(!board.uid.is_empty());
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn leaderboards_with_variant() {
    let api = api();
    let mut boards = api.get_leaderboards(
        1000008921,
        LeaderboardOptions::new().max_page(1).variant("120er"),
    );
    while let Some(board) = boards.next().await.unwrap() {
        assert!(!board.uid.is_empty());
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn leaderboard_total_size() {
    api().get_leaderboard_total_size(1000009808).await.unwrap();
}

#[tokio::test]
#[ignore = "requires network access"]
async fn stygian_leaderboards() {
    let api = api();
    let mut boards = api.get_stygian_leaderboards("5_8", StygianOptions::new().max_page(1));
    while let Some(board) = boards.next().await.unwrap() {
        assert!(!board.uid.is_empty());
        assert!(board.stygian_score > 0.0);
        assert!(board.stygian_seconds > 0);
        assert!(board.stygian_index > 0);
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn stygian_leaderboards_with_uids() {
    let api = api();
    let mut boards = api.get_stygian_leaderboards(
        "5_8",
        StygianOptions::new()
            .max_page(1)
            .uids([626792307, 730208419]),
    );
    while let Some(board) = boards.next().await.unwrap() {
        assert!(matches!(board.uid.as_str(), "626792307" | "730208419"));
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn stygian_leaderboard_page_size() {
    let api = api();
    let mut boards =
        api.get_stygian_leaderboards("5_8", StygianOptions::new().max_page(1).page_size(5));
    let mut count = 0;
    while boards.next().await.unwrap().is_some() {
        count += 1;
    }
    assert!(count <= 5);
}

#[tokio::test]
#[ignore = "requires network access"]
async fn profiles() {
    for sort_by in [
        ProfileSortBy::Stygian,
        ProfileSortBy::AchievementNum,
        ProfileSortBy::UpdatedAt,
    ] {
        for order_by in [OrderBy::Ascending, OrderBy::Descending] {
            let api = api();
            let mut profiles = api.get_profiles(
                ProfileOptions::new(sort_by)
                    .order_by(order_by)
                    .use_cache(false),
            );
            assert!(profiles.next().await.unwrap().is_some());
        }
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn profiles_with_pagination() {
    for sort_by in [
        ProfileSortBy::Stygian,
        ProfileSortBy::AchievementNum,
        ProfileSortBy::UpdatedAt,
    ] {
        for order_by in [OrderBy::Ascending, OrderBy::Descending] {
            let api = api();
            let mut profiles = api.get_profiles(
                ProfileOptions::new(sort_by)
                    .order_by(order_by)
                    .use_cache(false),
            );
            while let Some(profile) = profiles.next().await.unwrap() {
                assert!(!profile.id.is_empty());
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires network access"]
async fn user_calculations() {
    api().get_calculations_for_user(901211014).await.unwrap();
}

#[tokio::test]
#[ignore = "requires network access"]
async fn refresh_user() {
    api().refresh_user(901211014).await.unwrap();
}

#[tokio::test]
#[ignore = "requires network access"]
async fn translations() {
    for language in [
        Language::German,
        Language::French,
        Language::Spanish,
        Language::Italian,
        Language::Japanese,
        Language::Korean,
        Language::Portuguese,
        Language::Russian,
        Language::Thai,
        Language::Turkish,
        Language::Vietnamese,
        Language::ChineseSimplified,
        Language::ChineseTraditional,
    ] {
        let api = Akasha::new(language).unwrap();
        api.get_translations(&[
            "Kamisato Ayaka".to_owned(),
            "Mistsplitter Reforged".to_owned(),
        ])
        .await
        .unwrap();
        api.translate("Kamisato Ayaka").await.unwrap();
    }
}
