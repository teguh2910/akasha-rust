//! Shared application state.

use std::collections::HashMap;
use std::sync::Arc;

use akasha::{Akasha, Language};

use crate::config::Config;

/// State cloned into every request handler.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    clients: HashMap<Language, Akasha>,
    config: Config,
}

impl AppState {
    /// Builds one Akasha client per supported language.
    pub fn new(config: Config) -> akasha::Result<Self> {
        let mut clients = HashMap::with_capacity(Language::ALL.len());

        for language in Language::ALL {
            let client = Akasha::builder()
                .language(language)
                .base_url(config.base_url.clone())
                .cache_ttl(config.cache_ttl)
                .build()?;
            clients.insert(language, client);
        }

        Ok(Self {
            inner: Arc::new(Inner { clients, config }),
        })
    }

    /// The client for a language. Every language is pre-built, so this cannot
    /// fail.
    pub fn client(&self, language: Language) -> &Akasha {
        &self.inner.clients[&language]
    }

    /// The service configuration.
    pub fn config(&self) -> &Config {
        &self.inner.config
    }
}
