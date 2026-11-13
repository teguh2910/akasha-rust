//! Profile paginator.

use crate::client::Akasha;
use crate::enums::{OrderBy, ProfileSortBy};
use crate::errors::Result;
use crate::models::Profile;

/// Options for [`Akasha::get_profiles`].
#[derive(Debug, Clone)]
pub struct ProfileOptions {
    pub sort_by: ProfileSortBy,
    pub order_by: OrderBy,
    pub max_page: u32,
    pub page_size: u32,
    pub use_cache: bool,
}

impl ProfileOptions {
    pub fn new(sort_by: ProfileSortBy) -> Self {
        Self {
            sort_by,
            order_by: OrderBy::Descending,
            max_page: 1,
            page_size: 20,
            use_cache: true,
        }
    }

    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.order_by = order_by;
        self
    }

    pub fn max_page(mut self, max_page: u32) -> Self {
        self.max_page = max_page;
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }
}

/// Iterates over profiles, page by page.
pub struct ProfilePaginator {
    api: Akasha,
    sort_by: ProfileSortBy,
    order_by: OrderBy,
    page_size: u32,
    max_page: u32,
    use_cache: bool,

    profiles: Vec<Profile>,
    page: u32,
    index: usize,
}

impl ProfilePaginator {
    pub(crate) fn new(api: Akasha, options: ProfileOptions) -> Self {
        Self {
            api,
            sort_by: options.sort_by,
            order_by: options.order_by,
            page_size: options.page_size,
            max_page: options.max_page,
            use_cache: options.use_cache,
            profiles: Vec::new(),
            page: 1,
            index: 0,
        }
    }

    /// Fetch the next profile, or `None` when there are no more.
    pub async fn next(&mut self) -> Result<Option<Profile>> {
        if self.index + 1 >= self.profiles.len() {
            if self.page > self.max_page {
                return Ok(None);
            }

            let (p, from_id) = match self.profiles.last() {
                Some(profile) => {
                    let p = match profile.player_info.stygian_score {
                        Some(score) => format!("lt|{score:?}"),
                        None => "lt|None".to_owned(),
                    };
                    (p, profile.id.clone())
                }
                None => (String::new(), String::new()),
            };

            self.profiles = self
                .api
                .fetch_profiles(
                    self.sort_by,
                    self.order_by,
                    self.page,
                    self.page_size,
                    &p,
                    &from_id,
                    self.use_cache,
                )
                .await?;
            self.index = 0;
            self.page += 1;
        } else {
            self.index += 1;
        }

        Ok(self.profiles.get(self.index).cloned())
    }

    /// Collect every remaining profile into a vector.
    pub async fn collect_all(&mut self) -> Result<Vec<Profile>> {
        let mut profiles = Vec::new();
        while let Some(profile) = self.next().await? {
            profiles.push(profile);
        }
        Ok(profiles)
    }
}
