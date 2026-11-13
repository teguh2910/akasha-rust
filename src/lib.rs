//! Async API wrapper for [akasha.cv](https://akasha.cv/) written in Rust.
//!
//! This crate is a Rust port of the `akasha-py` Python wrapper. It exposes an
//! async [`Akasha`] client, typed models, and paginators for the list
//! endpoints.
//!
//! ```no_run
//! use akasha::{Akasha, Language, LeaderboardOptions};
//!
//! # async fn run() -> akasha::Result<()> {
//! let api = Akasha::new(Language::English)?;
//! let characters = api.get_calculations_for_user(901211014).await?;
//!
//! for character in characters {
//!     let calc = &character.calculations[0];
//!     println!("Character: {}", character.name);
//!     println!("Top {:.2}%", calc.top_percent());
//!
//!     let mut boards =
//!         api.get_leaderboards(calc.id, LeaderboardOptions::new().max_page(1).page_size(3));
//!     while let Some(board) = boards.next().await? {
//!         println!("{}. {}", board.rank, board.owner.nickname);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod constants;
pub mod enums;
pub mod errors;
pub mod models;
pub mod paginators;

pub use client::{Akasha, AkashaBuilder, AkashaConfig, BASE_URL};
pub use constants::{
    is_percent_artifact_stat, is_percent_chara_stat, PERCENT_ARTIFACT_STATS, PERCENT_CHARA_STATS,
};
pub use enums::{
    ArtifactStat as ArtifactStatType, CharaStatType, Element, EquipType, Language, OrderBy,
    ProfileSortBy,
};
pub use errors::{Error, Result};
pub use models::*;
pub use paginators::{
    LeaderboardOptions, LeaderboardPaginator, ProfileOptions, ProfilePaginator,
    StygianLeaderboardPaginator, StygianOptions,
};
