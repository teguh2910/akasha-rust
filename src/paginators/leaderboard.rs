//! Leaderboard paginators.

use crate::client::Akasha;
use crate::errors::Result;
use crate::models::{Leaderboard, StygianLeaderboard};

/// Options for [`Akasha::get_leaderboards`].
#[derive(Debug, Clone)]
pub struct LeaderboardOptions {
    pub max_page: u32,
    pub page_size: u32,
    pub variant: Option<String>,
    pub uids: Option<Vec<u64>>,
    pub use_cache: bool,
}

impl Default for LeaderboardOptions {
    fn default() -> Self {
        Self {
            max_page: 1,
            page_size: 20,
            variant: None,
            uids: None,
            use_cache: true,
        }
    }
}

impl LeaderboardOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_page(mut self, max_page: u32) -> Self {
        self.max_page = max_page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    pub fn uids(mut self, uids: impl IntoIterator<Item = u64>) -> Self {
        self.uids = Some(uids.into_iter().collect());
        self
    }

    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }
}

/// Iterates over the leaderboards of a calculation, page by page.
pub struct LeaderboardPaginator {
    api: Akasha,
    calculation_id: u64,
    page_size: u32,
    max_page: u32,
    variant: Option<String>,
    uids: Option<Vec<u64>>,
    use_cache: bool,

    boards: Vec<Leaderboard>,
    page: u32,
    index: usize,
}

impl LeaderboardPaginator {
    pub(crate) fn new(api: Akasha, calculation_id: u64, options: LeaderboardOptions) -> Self {
        Self {
            api,
            calculation_id,
            page_size: options.page_size,
            max_page: options.max_page,
            variant: options.variant,
            uids: options.uids,
            use_cache: options.use_cache,
            boards: Vec::new(),
            page: 1,
            index: 0,
        }
    }

    /// Fetch the next leaderboard entry, or `None` when there are no more.
    pub async fn next(&mut self) -> Result<Option<Leaderboard>> {
        if self.index + 1 >= self.boards.len() {
            if self.page > self.max_page {
                return Ok(None);
            }

            let p = match self.boards.last() {
                Some(board) => format!("lt|{:?}", board.calculation.result),
                None => String::new(),
            };

            self.boards = self
                .api
                .fetch_leaderboards(
                    self.calculation_id,
                    self.page,
                    self.page_size,
                    &p,
                    self.variant.as_deref(),
                    self.uids.as_deref(),
                    self.use_cache,
                )
                .await?;
            self.index = 0;
            self.page += 1;
        } else {
            self.index += 1;
        }

        Ok(self.boards.get(self.index).cloned())
    }

    /// Collect every remaining leaderboard entry into a vector.
    pub async fn collect_all(&mut self) -> Result<Vec<Leaderboard>> {
        let mut boards = Vec::new();
        while let Some(board) = self.next().await? {
            boards.push(board);
        }
        Ok(boards)
    }
}

/// Options for [`Akasha::get_stygian_leaderboards`].
#[derive(Debug, Clone)]
pub struct StygianOptions {
    pub max_page: u32,
    pub page_size: u32,
    pub uids: Option<Vec<u64>>,
    pub use_cache: bool,
}

impl Default for StygianOptions {
    fn default() -> Self {
        Self {
            max_page: 1,
            page_size: 20,
            uids: None,
            use_cache: true,
        }
    }
}

impl StygianOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_page(mut self, max_page: u32) -> Self {
        self.max_page = max_page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn uids(mut self, uids: impl IntoIterator<Item = u64>) -> Self {
        self.uids = Some(uids.into_iter().collect());
        self
    }

    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }
}

/// Iterates over Stygian Onslaught leaderboards, page by page.
pub struct StygianLeaderboardPaginator {
    api: Akasha,
    version: String,
    page_size: u32,
    max_page: u32,
    uids: Option<Vec<u64>>,
    use_cache: bool,

    boards: Vec<StygianLeaderboard>,
    page: u32,
    index: usize,
}

impl StygianLeaderboardPaginator {
    pub(crate) fn new(api: Akasha, version: String, options: StygianOptions) -> Self {
        Self {
            api,
            version,
            page_size: options.page_size,
            max_page: options.max_page,
            uids: options.uids,
            use_cache: options.use_cache,
            boards: Vec::new(),
            page: 1,
            index: 0,
        }
    }

    /// Fetch the next leaderboard entry, or `None` when there are no more.
    pub async fn next(&mut self) -> Result<Option<StygianLeaderboard>> {
        if self.index + 1 >= self.boards.len() {
            if self.page > self.max_page {
                return Ok(None);
            }

            let p = match self.boards.last() {
                Some(board) => format!("lt|{:?}", board.stygian_score),
                None => String::new(),
            };

            self.boards = self
                .api
                .fetch_stygian_leaderboards(
                    self.page,
                    self.page_size,
                    &p,
                    &self.version,
                    self.uids.as_deref(),
                    self.use_cache,
                )
                .await?;
            self.index = 0;
            self.page += 1;
        } else {
            self.index += 1;
        }

        Ok(self.boards.get(self.index).cloned())
    }

    /// Collect every remaining leaderboard entry into a vector.
    pub async fn collect_all(&mut self) -> Result<Vec<StygianLeaderboard>> {
        let mut boards = Vec::new();
        while let Some(board) = self.next().await? {
            boards.push(board);
        }
        Ok(boards)
    }
}
