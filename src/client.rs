//! The asynchronous Akasha API client.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::debug;

use crate::enums::{Language, OrderBy, ProfileSortBy};
use crate::errors::{error_from_description, Error, Result};
use crate::models::{
    Artifact, Leaderboard, LeaderboardCategory, Profile, StygianLeaderboard, UserCalc,
};
use crate::paginators::{
    LeaderboardOptions, LeaderboardPaginator, ProfileOptions, ProfilePaginator,
    StygianLeaderboardPaginator, StygianOptions,
};

pub const BASE_URL: &str = "https://akasha.cv/api";

/// Configuration used to build an [`Akasha`] client.
#[derive(Debug, Clone)]
pub struct AkashaConfig {
    pub language: Language,
    pub headers: Option<HeaderMap>,
    pub cache_ttl: Duration,
    pub debug: bool,
}

impl Default for AkashaConfig {
    fn default() -> Self {
        Self {
            language: Language::English,
            headers: None,
            cache_ttl: Duration::from_secs(360),
            debug: false,
        }
    }
}

/// Builder for [`Akasha`].
#[derive(Debug, Clone)]
pub struct AkashaBuilder {
    config: AkashaConfig,
}

impl AkashaBuilder {
    pub fn language(mut self, language: Language) -> Self {
        self.config.language = language;
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.config.headers = Some(headers);
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        let name =
            HeaderName::from_bytes(name.as_bytes()).map_err(|e| Error::Header(e.to_string()))?;
        let value = HeaderValue::from_str(value).map_err(|e| Error::Header(e.to_string()))?;

        let headers = self.config.headers.get_or_insert_with(HeaderMap::new);
        headers.insert(name, value);
        Ok(self)
    }

    pub fn cache_ttl(mut self, cache_ttl: Duration) -> Self {
        self.config.cache_ttl = cache_ttl;
        self
    }

    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }

    pub fn build(self) -> Result<Akasha> {
        Akasha::from_config(self.config)
    }
}

struct CacheEntry {
    value: Value,
    expires_at: Instant,
}

struct Inner {
    client: reqwest::Client,
    config: AkashaConfig,
    cache: Mutex<HashMap<String, CacheEntry>>,
}

/// Async client for the akasha.cv API.
///
/// Cloning the client is cheap; clones share the same underlying HTTP client
/// and response cache.
#[derive(Clone)]
pub struct Akasha {
    inner: Arc<Inner>,
}

impl Akasha {
    /// Create a builder with default configuration.
    pub fn builder() -> AkashaBuilder {
        AkashaBuilder {
            config: AkashaConfig::default(),
        }
    }

    /// Create a client with the given language and default options.
    pub fn new(language: Language) -> Result<Self> {
        Self::builder().language(language).build()
    }

    fn from_config(config: AkashaConfig) -> Result<Self> {
        let mut headers = config.headers.clone().unwrap_or_default();
        if headers.is_empty() {
            headers.insert(USER_AGENT, HeaderValue::from_static("akasha-py"));
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            inner: Arc::new(Inner {
                client,
                config,
                cache: Mutex::new(HashMap::new()),
            }),
        })
    }

    pub fn language(&self) -> Language {
        self.inner.config.language
    }

    pub fn cache_ttl(&self) -> Duration {
        self.inner.config.cache_ttl
    }

    async fn request(
        &self,
        endpoint: &str,
        use_cache: bool,
        params: Vec<(&str, String)>,
        return_raw: bool,
    ) -> Result<Value> {
        let mut url =
            Url::parse(&format!("{BASE_URL}/{endpoint}")).map_err(|e| Error::Url(e.to_string()))?;

        if !params.is_empty() {
            let mut query = url.query_pairs_mut();
            for (key, value) in &params {
                query.append_pair(key, value);
            }
        }

        let cache_key = url.to_string();

        if use_cache {
            let cache = self.inner.cache.lock().unwrap();
            if let Some(entry) = cache.get(&cache_key) {
                if entry.expires_at > Instant::now() {
                    return Ok(entry.value.clone());
                }
            }
        }

        if self.inner.config.debug {
            debug!("Requesting {url} with params {params:?}");
        }

        let response = self
            .inner
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        let value: Value = response.json().await?;

        if use_cache {
            let mut cache = self.inner.cache.lock().unwrap();
            cache.insert(
                cache_key,
                CacheEntry {
                    value: value.clone(),
                    expires_at: Instant::now() + self.inner.config.cache_ttl,
                },
            );
        }

        if return_raw {
            return Ok(value);
        }

        let data = value.get("data").cloned().unwrap_or(value);
        raise_for_error(&data)?;
        Ok(data)
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        use_cache: bool,
        params: Vec<(&str, String)>,
    ) -> Result<T> {
        let value = self.request(endpoint, use_cache, params, false).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Get the calculations for a user.
    pub async fn get_calculations_for_user(&self, uid: u64) -> Result<Vec<UserCalc>> {
        self.get_calculations_for_user_opts(uid, true).await
    }

    /// Get the calculations for a user, optionally bypassing the cache.
    pub async fn get_calculations_for_user_opts(
        &self,
        uid: u64,
        use_cache: bool,
    ) -> Result<Vec<UserCalc>> {
        let data = self
            .request(
                &format!("getCalculationsForUser/{uid}"),
                use_cache,
                Vec::new(),
                false,
            )
            .await?;
        let mut user_calcs: Vec<UserCalc> = serde_json::from_value(data)?;

        let mut strings = HashSet::new();
        for user_calc in &user_calcs {
            strings.insert(user_calc.name.clone());
            for calc in &user_calc.calculations {
                strings.insert(calc.weapon.name.clone());
            }
        }

        let words: Vec<String> = strings.into_iter().collect();
        let translations = self.get_translations(&words).await?;

        for user_calc in &mut user_calcs {
            if let Some(translation) = translations.get(&user_calc.name.to_lowercase()) {
                user_calc.name = translation.clone();
            }
            for calc in &mut user_calc.calculations {
                if let Some(translation) = translations.get(&calc.weapon.name.to_lowercase()) {
                    calc.weapon.name = translation.clone();
                }
            }
        }

        Ok(user_calcs)
    }

    /// Get the translations for a list of words.
    ///
    /// The returned map is keyed by the lowercased word.
    pub async fn get_translations(&self, words: &[String]) -> Result<HashMap<String, String>> {
        self.get_translations_opts(words, true).await
    }

    /// Get the translations for a list of words, optionally bypassing the cache.
    pub async fn get_translations_opts(
        &self,
        words: &[String],
        use_cache: bool,
    ) -> Result<HashMap<String, String>> {
        if self.inner.config.language == Language::English {
            return Ok(words
                .iter()
                .map(|word| (word.to_lowercase(), word.clone()))
                .collect());
        }

        let params: Vec<(&str, String)> =
            words.iter().map(|word| ("words[]", word.clone())).collect();

        let data = self
            .request(
                &format!("textmap/{}", self.inner.config.language.as_str()),
                use_cache,
                params,
                false,
            )
            .await?;

        let translation = data
            .get("translation")
            .ok_or(Error::MissingField("translation"))?;
        Ok(serde_json::from_value(translation.clone())?)
    }

    /// Translate a single word.
    ///
    /// Use [`Akasha::get_translations`] instead when translating multiple words.
    pub async fn translate(&self, word: &str) -> Result<String> {
        if self.inner.config.language == Language::English {
            return Ok(word.to_owned());
        }

        let mut translations = self.get_translations(&[word.to_owned()]).await?;
        translations
            .remove(&word.to_lowercase())
            .ok_or_else(|| Error::MissingTranslation(word.to_owned()))
    }

    /// Get the artifacts of a build.
    pub async fn get_artifacts(&self, uid: u64, md5: &str) -> Result<Vec<Artifact>> {
        let value = self
            .request(&format!("artifacts/{uid}/{md5}"), true, Vec::new(), false)
            .await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Get the leaderboard categories of a character.
    pub async fn get_categories(&self, character_id: &str) -> Result<Vec<LeaderboardCategory>> {
        self.get_json(
            "v2/leaderboards/categories",
            true,
            vec![("characterId", character_id.to_owned())],
        )
        .await
    }

    /// Refresh the Enka data of a player.
    pub async fn refresh_user(&self, uid: u64) -> Result<()> {
        self.request(&format!("user/refresh/{uid}"), false, Vec::new(), false)
            .await?;
        Ok(())
    }

    /// Not implemented by the upstream API wrapper.
    pub async fn get_user(&self, _uid: u64) -> Result<()> {
        Err(Error::NotImplemented("get_user"))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn fetch_leaderboards(
        &self,
        calculation_id: u64,
        page: u32,
        page_size: u32,
        p: &str,
        variant: Option<&str>,
        uids: Option<&[u64]>,
        use_cache: bool,
    ) -> Result<Vec<Leaderboard>> {
        let has_uids = uids.is_some_and(|uids| !uids.is_empty());
        let uids_param = if has_uids {
            let joined = uids
                .unwrap_or_default()
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join("[uid]");
            format!("[uid]{joined}")
        } else {
            String::new()
        };

        let params = vec![
            ("calculationId", calculation_id.to_string()),
            ("size", page_size.to_string()),
            ("page", page.to_string()),
            ("sort", "calculation.result".to_owned()),
            ("order", "-1".to_owned()),
            ("variant", variant.unwrap_or_default().to_owned()),
            ("p", p.to_owned()),
            ("uids", uids_param),
            (
                "filter",
                if has_uids {
                    "[all]1".to_owned()
                } else {
                    String::new()
                },
            ),
        ];

        let value = self
            .request("leaderboards", use_cache, params, false)
            .await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Get a leaderboard paginator for a calculation.
    pub fn get_leaderboards(
        &self,
        calculation_id: u64,
        options: LeaderboardOptions,
    ) -> LeaderboardPaginator {
        LeaderboardPaginator::new(self.clone(), calculation_id, options)
    }

    /// Get the size of a collection.
    pub async fn get_collection_size(&self, hash: &str, variant: &str) -> Result<u64> {
        let value = self
            .request(
                "getCollectionSize",
                true,
                vec![("hash", hash.to_owned()), ("variant", variant.to_owned())],
                true,
            )
            .await?;

        value
            .get("totalRows")
            .and_then(|rows| {
                rows.as_u64()
                    .or_else(|| rows.as_i64().map(|rows| rows as u64))
            })
            .ok_or(Error::MissingField("totalRows"))
    }

    /// Get the total size of a leaderboard.
    pub async fn get_leaderboard_total_size(&self, calculation_id: u64) -> Result<u64> {
        let params = vec![
            ("calculationId", calculation_id.to_string()),
            ("sort", "calculation.result".to_owned()),
            ("order", "-1".to_owned()),
            ("size", "1".to_owned()),
            ("page", "1".to_owned()),
        ];

        let value = self.request("leaderboards", true, params, true).await?;
        let hash = value
            .get("totalRowsHash")
            .and_then(Value::as_str)
            .ok_or(Error::MissingField("totalRowsHash"))?
            .to_owned();

        self.get_collection_size(&hash, "charactersLb").await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn fetch_profiles(
        &self,
        sort_by: ProfileSortBy,
        order_by: OrderBy,
        page: u32,
        page_size: u32,
        p: &str,
        from_id: &str,
        use_cache: bool,
    ) -> Result<Vec<Profile>> {
        let params = vec![
            ("sort", sort_by.as_str().to_owned()),
            ("order", order_by.value().to_string()),
            ("page", page.to_string()),
            ("pageSize", page_size.to_string()),
            ("p", p.to_owned()),
            ("fromId", from_id.to_owned()),
        ];

        let value = self.request("accounts", use_cache, params, false).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Get a paginator for profiles.
    pub fn get_profiles(&self, options: ProfileOptions) -> ProfilePaginator {
        ProfilePaginator::new(self.clone(), options)
    }

    pub(crate) async fn fetch_stygian_leaderboards(
        &self,
        page: u32,
        page_size: u32,
        p: &str,
        version: &str,
        uids: Option<&[u64]>,
        use_cache: bool,
    ) -> Result<Vec<StygianLeaderboard>> {
        let has_uids = uids.is_some_and(|uids| !uids.is_empty());
        let uids_param = if has_uids {
            let joined = uids
                .unwrap_or_default()
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join("[uid]");
            format!("[uid]{joined}")
        } else {
            String::new()
        };

        let params = vec![
            ("size", page_size.to_string()),
            ("page", page.to_string()),
            ("sort", "stygianScore".to_owned()),
            ("order", "-1".to_owned()),
            ("version", version.to_owned()),
            ("p", p.to_owned()),
            ("uids", uids_param),
            (
                "filter",
                if has_uids {
                    "[susLevel]1[all]1".to_owned()
                } else {
                    String::new()
                },
            ),
        ];

        let value = self
            .request("leaderboards/stygian", use_cache, params, false)
            .await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Get a Stygian leaderboard paginator.
    pub fn get_stygian_leaderboards(
        &self,
        version: impl Into<String>,
        options: StygianOptions,
    ) -> StygianLeaderboardPaginator {
        StygianLeaderboardPaginator::new(self.clone(), version.into(), options)
    }
}

fn raise_for_error(data: &Value) -> Result<()> {
    if data.is_array() {
        return Ok(());
    }

    let Some(object) = data.as_object() else {
        return Ok(());
    };

    let message = object.get("message").and_then(Value::as_str);

    if let Some(message) = message {
        if message.contains("I want") {
            return Err(Error::InvalidApiRequest {
                title: "Invalid API request".to_owned(),
                description: message.to_owned(),
            });
        }
    }

    let Some(error) = object.get("error") else {
        return Ok(());
    };

    let (title, description) = match message {
        Some(message) => {
            let description = match error {
                Value::String(description) => description.clone(),
                other => other.to_string(),
            };
            (message.to_owned(), description)
        }
        None => match error {
            Value::Object(error) => (
                error
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                error
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
            ),
            other => (String::new(), other.to_string()),
        },
    };

    Err(error_from_description(title, description))
}
