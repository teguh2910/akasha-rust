//! HTTP routes.

use akasha::{
    Artifact, Language, Leaderboard, LeaderboardCategory, OrderBy, Profile, ProfileSortBy,
    StygianLeaderboard, UserCalc,
};
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_http::trace::TraceLayer;

use crate::error::ApiError;
use crate::state::AppState;

/// Page size used when the caller does not ask for one.
const DEFAULT_PAGE_SIZE: u32 = 20;
/// Largest page the service will request from upstream.
const MAX_PAGE_SIZE: u32 = 100;

/// Builds the router with all routes and the shared state attached.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/v1/health", get(health))
        .route("/v1/users/{uid}/characters", get(user_characters))
        .route("/v1/users/{uid}/artifacts/{md5}", get(user_artifacts))
        .route("/v1/characters/{character_id}/categories", get(categories))
        .route("/v1/leaderboards/{calculation_id}", get(leaderboards))
        .route("/v1/profiles", get(profiles))
        .route("/v1/stygian/{version}", get(stygian_leaderboards))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn index() -> Json<Value> {
    Json(json!({
        "name": "akasha-service",
        "version": env!("CARGO_PKG_VERSION"),
        "source": "https://akasha.cv",
        "endpoints": [
            "GET /health",
            "GET /v1/users/{uid}/characters?lang=en&fresh=false",
            "GET /v1/users/{uid}/artifacts/{md5}?lang=en",
            "GET /v1/characters/{character_id}/categories?lang=en",
            "GET /v1/leaderboards/{calculation_id}?lang=en&size=20&cursor=&variant=&uids=&fresh=false",
            "GET /v1/profiles?lang=en&sort=stygian&order=desc&size=20&cursor=&from_id=",
            "GET /v1/stygian/{version}?lang=en&size=20&cursor=&uids=&fresh=false"
        ]
    }))
}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[derive(Debug, Deserialize)]
struct LangQuery {
    lang: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserQuery {
    lang: Option<String>,
    fresh: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LeaderboardQuery {
    lang: Option<String>,
    #[serde(alias = "page_size")]
    size: Option<String>,
    cursor: Option<String>,
    variant: Option<String>,
    uids: Option<String>,
    fresh: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfileQuery {
    lang: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    #[serde(alias = "page_size")]
    size: Option<String>,
    cursor: Option<String>,
    #[serde(alias = "fromId")]
    from_id: Option<String>,
    fresh: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StygianQuery {
    lang: Option<String>,
    #[serde(alias = "page_size")]
    size: Option<String>,
    cursor: Option<String>,
    uids: Option<String>,
    fresh: Option<String>,
}

async fn user_characters(
    State(state): State<AppState>,
    Path(uid): Path<String>,
    Query(query): Query<UserQuery>,
) -> Result<Json<Value>, ApiError> {
    let uid = parse_id(&uid, "uid")?;
    let api = state.client(parse_language(query.lang.as_deref())?);
    let use_cache = !parse_fresh(query.fresh.as_deref())?;

    let data: Vec<UserCalc> = api.get_calculations_for_user_opts(uid, use_cache).await?;
    Ok(Json(json!({ "data": data })))
}

async fn user_artifacts(
    State(state): State<AppState>,
    Path((uid, md5)): Path<(String, String)>,
    Query(query): Query<LangQuery>,
) -> Result<Json<Value>, ApiError> {
    let uid = parse_id(&uid, "uid")?;
    let api = state.client(parse_language(query.lang.as_deref())?);

    let data: Vec<Artifact> = api.get_artifacts(uid, &md5).await?;
    Ok(Json(json!({ "data": data })))
}

async fn categories(
    State(state): State<AppState>,
    Path(character_id): Path<String>,
    Query(query): Query<LangQuery>,
) -> Result<Json<Value>, ApiError> {
    let api = state.client(parse_language(query.lang.as_deref())?);

    let data: Vec<LeaderboardCategory> = api.get_categories(&character_id).await?;
    Ok(Json(json!({ "data": data })))
}

async fn leaderboards(
    State(state): State<AppState>,
    Path(calculation_id): Path<String>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<Value>, ApiError> {
    let calculation_id = parse_id(&calculation_id, "calculation_id")?;
    let api = state.client(parse_language(query.lang.as_deref())?);
    let size = parse_size(query.size.as_deref())?;
    let cursor = parse_cursor(query.cursor.as_deref())?;
    let variant = query.variant.as_deref().filter(|value| !value.is_empty());
    let uids = parse_uids(query.uids.as_deref())?;
    let use_cache = !parse_fresh(query.fresh.as_deref())?;

    let data: Vec<Leaderboard> = api
        .get_leaderboards_page(
            calculation_id,
            size,
            &cursor,
            variant,
            uids.as_deref(),
            use_cache,
        )
        .await?;

    let next_cursor = data
        .last()
        .map(|board| board.calculation.result.to_string());

    Ok(Json(json!({ "data": data, "next_cursor": next_cursor })))
}

async fn profiles(
    State(state): State<AppState>,
    Query(query): Query<ProfileQuery>,
) -> Result<Json<Value>, ApiError> {
    let api = state.client(parse_language(query.lang.as_deref())?);
    let sort_by = parse_sort(query.sort.as_deref())?;
    let order_by = parse_order(query.order.as_deref())?;
    let size = parse_size(query.size.as_deref())?;
    let cursor = parse_cursor(query.cursor.as_deref())?;
    let from_id = query.from_id.unwrap_or_default();
    let use_cache = !parse_fresh(query.fresh.as_deref())?;

    let data: Vec<Profile> = api
        .get_profiles_page(sort_by, order_by, size, &cursor, &from_id, use_cache)
        .await?;

    let next_cursor = data
        .last()
        .and_then(|profile| profile_cursor(profile, sort_by));
    let next_from_id = data.last().map(|profile| profile.id.clone());

    Ok(Json(
        json!({ "data": data, "next_cursor": next_cursor, "next_from_id": next_from_id }),
    ))
}

async fn stygian_leaderboards(
    State(state): State<AppState>,
    Path(version): Path<String>,
    Query(query): Query<StygianQuery>,
) -> Result<Json<Value>, ApiError> {
    let api = state.client(parse_language(query.lang.as_deref())?);
    let size = parse_size(query.size.as_deref())?;
    let cursor = parse_cursor(query.cursor.as_deref())?;
    let uids = parse_uids(query.uids.as_deref())?;
    let use_cache = !parse_fresh(query.fresh.as_deref())?;

    let data: Vec<StygianLeaderboard> = api
        .get_stygian_leaderboards_page(&version, size, &cursor, uids.as_deref(), use_cache)
        .await?;

    let next_cursor = data.last().map(|board| board.stygian_score.to_string());

    Ok(Json(json!({ "data": data, "next_cursor": next_cursor })))
}

/// The cursor the caller has to send back to continue pagination.
///
/// Achievement counts lose their fractional tiebreaker when deserialized, so
/// that sort cannot produce a cursor automatically.
fn profile_cursor(profile: &Profile, sort_by: ProfileSortBy) -> Option<String> {
    match sort_by {
        ProfileSortBy::Stygian => profile
            .player_info
            .stygian_score
            .map(|score| score.to_string()),
        ProfileSortBy::AchievementNum => None,
        ProfileSortBy::UpdatedAt => profile
            .updated_at
            .map(|updated_at| updated_at.timestamp_millis().to_string()),
    }
}

fn parse_id(raw: &str, field: &str) -> Result<u64, ApiError> {
    raw.parse()
        .map_err(|_| ApiError::bad_request(format!("{field} must be a positive integer")))
}

fn parse_language(raw: Option<&str>) -> Result<Language, ApiError> {
    let Some(raw) = raw else {
        return Ok(Language::English);
    };

    Language::ALL
        .into_iter()
        .find(|language| language.as_str() == raw)
        .ok_or_else(|| ApiError::bad_request(format!("unsupported lang `{raw}`")))
}

fn parse_fresh(raw: Option<&str>) -> Result<bool, ApiError> {
    match raw {
        None | Some("false") | Some("0") => Ok(false),
        Some("true") | Some("1") => Ok(true),
        Some(other) => Err(ApiError::bad_request(format!(
            "fresh must be true or false, got `{other}`"
        ))),
    }
}

fn parse_size(raw: Option<&str>) -> Result<u32, ApiError> {
    let Some(raw) = raw else {
        return Ok(DEFAULT_PAGE_SIZE);
    };

    let size: u32 = raw
        .parse()
        .map_err(|_| ApiError::bad_request("size must be a whole number"))?;

    if size == 0 || size > MAX_PAGE_SIZE {
        return Err(ApiError::bad_request(format!(
            "size must be between 1 and {MAX_PAGE_SIZE}"
        )));
    }

    Ok(size)
}

fn parse_cursor(raw: Option<&str>) -> Result<String, ApiError> {
    let Some(raw) = raw else {
        return Ok(String::new());
    };

    if raw.is_empty() {
        return Ok(String::new());
    }

    raw.parse::<f64>()
        .map_err(|_| ApiError::bad_request("cursor must be a number"))?;
    Ok(raw.to_owned())
}

fn parse_uids(raw: Option<&str>) -> Result<Option<Vec<u64>>, ApiError> {
    let Some(raw) = raw else {
        return Ok(None);
    };

    let uids = raw
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse()
                .map_err(|_| ApiError::bad_request("uids must be comma-separated integers"))
        })
        .collect::<Result<Vec<u64>, _>>()?;

    if uids.is_empty() {
        Ok(None)
    } else {
        Ok(Some(uids))
    }
}

fn parse_sort(raw: Option<&str>) -> Result<ProfileSortBy, ApiError> {
    match raw.unwrap_or("stygian") {
        "stygian" | "score" => Ok(ProfileSortBy::Stygian),
        "achievements" | "achievement" => Ok(ProfileSortBy::AchievementNum),
        "updated" | "updated_at" => Ok(ProfileSortBy::UpdatedAt),
        other => Err(ApiError::bad_request(format!(
            "unsupported sort `{other}`, expected stygian, achievements or updated"
        ))),
    }
}

fn parse_order(raw: Option<&str>) -> Result<OrderBy, ApiError> {
    match raw.unwrap_or("desc") {
        "asc" | "ascending" => Ok(OrderBy::Ascending),
        "desc" | "descending" => Ok(OrderBy::Descending),
        other => Err(ApiError::bad_request(format!(
            "unsupported order `{other}`, expected asc or desc"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_defaults_to_english() {
        assert_eq!(parse_language(None).unwrap(), Language::English);
        assert_eq!(
            parse_language(Some("zh-CN")).unwrap(),
            Language::ChineseSimplified
        );
        assert!(parse_language(Some("xx")).is_err());
    }

    #[test]
    fn size_is_bounded() {
        assert_eq!(parse_size(None).unwrap(), DEFAULT_PAGE_SIZE);
        assert_eq!(parse_size(Some("5")).unwrap(), 5);
        assert!(parse_size(Some("0")).is_err());
        assert!(parse_size(Some("101")).is_err());
        assert!(parse_size(Some("abc")).is_err());
    }

    #[test]
    fn uids_are_parsed_and_validated() {
        assert_eq!(parse_uids(None).unwrap(), None);
        assert_eq!(parse_uids(Some(" ")).unwrap(), None);
        assert_eq!(parse_uids(Some("1, 2,3")).unwrap(), Some(vec![1, 2, 3]));
        assert!(parse_uids(Some("1,two")).is_err());
    }

    #[test]
    fn cursor_is_numeric() {
        assert_eq!(parse_cursor(None).unwrap(), "");
        assert_eq!(parse_cursor(Some("12.5")).unwrap(), "12.5");
        assert!(parse_cursor(Some("lt|12.5")).is_err());
    }

    #[test]
    fn sort_and_order_are_mapped() {
        assert_eq!(parse_sort(None).unwrap(), ProfileSortBy::Stygian);
        assert_eq!(
            parse_sort(Some("updated")).unwrap(),
            ProfileSortBy::UpdatedAt
        );
        assert!(parse_sort(Some("name")).is_err());
        assert_eq!(parse_order(None).unwrap(), OrderBy::Descending);
        assert_eq!(parse_order(Some("asc")).unwrap(), OrderBy::Ascending);
        assert!(parse_order(Some("up")).is_err());
    }
}
