# akasha-rs

## Introduction

akasha-rs is an async API wrapper for [akasha.cv](https://akasha.cv/) written in Rust.
akasha.cv is a Genshin Impact leaderboard website.
This crate is a Rust rewrite of the original [akasha-py](https://github.com/seriaati/akasha-py) Python wrapper.
Developing something for Hoyoverse games? You might be interested in [other API wrappers](https://projects.seria.moe/projects/#wrapper) written by the original author.
> Note: I am not the developer of akasha.cv.

## Important Notes

A couple things I want to address:

- This wrapper is currently in-progress, some features may not work as expected, many features are also missing.
- The developer of akasha.cv mentioned that they change their API very frequently (and without notice), so this wrapper may break at any time.
- The original Python implementation is still present in `akasha/` for reference. The Rust crate lives in `src/`.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
akasha = { git = "https://github.com/seriaati/akasha-py" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Usage

Interact with the API using the `akasha::Akasha` client.
You can find all available methods in [`src/client.rs`](src/client.rs).
A quick example is provided below:

```rust
use akasha::{Akasha, Language, LeaderboardOptions};

#[tokio::main]
async fn main() -> akasha::Result<()> {
    let uid = 901211014;
    let api = Akasha::new(Language::ChineseSimplified)?;

    let characters = api.get_calculations_for_user(uid).await?;
    for character in characters {
        let Some(calc) = character.calculations.first() else {
            continue;
        };

        println!("Character: {}", character.name);
        println!("Weapon: {}", calc.weapon.name);
        println!(
            "Top {:.2}% ({}/{})",
            calc.top_percent(),
            calc.ranking,
            calc.out_of
        );
        println!("Damage: {}", calc.result.round());
        println!();

        println!("Leaderboard top 3:");
        let mut boards =
            api.get_leaderboards(calc.id, LeaderboardOptions::new().max_page(1).page_size(3));
        while let Some(board) = boards.next().await? {
            println!(
                "{}. {} | Damage: {}",
                board.rank,
                board.owner.nickname,
                board.calculation.result.round()
            );
        }

        println!("Leaderboard URL: https://akasha.cv/leaderboards/{}", calc.id);
        println!("{}", "=".repeat(50));
    }

    Ok(())
}
```

Run it with:

```bash
cargo run --example basic
```

## HTTP service

`crates/akasha-service` exposes the client as a small JSON API. It is deployed at
`https://akas.ttmi.my.id`.

```bash
# Build and run locally (listens on 0.0.0.0:8099 by default)
cargo run -p akasha-service
```

Endpoints:

| Method | Path | Query parameters |
| --- | --- | --- |
| GET | `/health` | - |
| GET | `/v1/health` | - |
| GET | `/v1/users/{uid}/characters` | `lang`, `fresh` |
| GET | `/v1/users/{uid}/artifacts/{md5}` | `lang` |
| GET | `/v1/characters/{character_id}/categories` | `lang` |
| GET | `/v1/leaderboards/{calculation_id}` | `lang`, `size`, `cursor`, `variant`, `uids`, `fresh` |
| GET | `/v1/profiles` | `lang`, `sort`, `order`, `size`, `cursor`, `from_id`, `fresh` |
| GET | `/v1/stygian/{version}` | `lang`, `size`, `cursor`, `uids`, `fresh` |

Query parameters:

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| `lang` | string | `en` | `en`, `de`, `fr`, `es`, `it`, `ja`, `ko`, `pt`, `ru`, `th`, `tr`, `vi`, `zh-CN`, `zh-TW` |
| `size` | integer | `20` | Page size, `1`-`100` |
| `cursor` | number | empty | Value from `next_cursor`; empty starts at the top |
| `from_id` | string | empty | Value from `next_from_id`; profiles only |
| `variant` | string | empty | Leaderboard variant filter |
| `uids` | string | empty | Comma-separated UIDs to filter by |
| `sort` | string | `stygian` | Profiles: `stygian`, `achievements`, `updated` |
| `order` | string | `desc` | `asc` or `desc` |
| `fresh` | boolean | `false` | `true` skips the in-process cache |

Every response is JSON. Single-resource endpoints return `{ "data": ... }`; list
endpoints also return `next_cursor` (and `next_from_id` for profiles), which you
pass back as `cursor` / `from_id` to read the next page.

```bash
# Player characters
curl 'https://akas.ttmi.my.id/v1/users/901211014/characters?lang=en'

# First leaderboard page, then the next one
curl 'https://akas.ttmi.my.id/v1/leaderboards/1000000216?size=20'
curl 'https://akas.ttmi.my.id/v1/leaderboards/1000000216?size=20&cursor=1282729.6509002843'

# Profiles sorted by last update
curl 'https://akas.ttmi.my.id/v1/profiles?sort=updated&size=20'

# Stygian Onslaught
curl 'https://akas.ttmi.my.id/v1/stygian/5_8?size=20'
```

> akasha.cv answers this host with a Cloudflare challenge when the TLS
> fingerprint looks like a plain HTTP client. The service therefore points at
> `scripts/akasha_fetch_proxy.py`, a loopback-only Python proxy that uses a
> browser fingerprint via `curl_cffi`. Set `AKASHA_BASE_URL` to change the
> upstream root. See [`deploy/README.md`](deploy/README.md).

## Differences from the Python wrapper

- Paginators expose an async `next()` that returns `Result<Option<T>>`; `collect_all()` collects every remaining item.
- Responses are cached in memory with a configurable TTL (`Akasha::builder().cache_ttl(...)`) instead of a SQLite database.
- Logging uses the [`tracing`](https://docs.rs/tracing) crate instead of `loguru`.

## Testing

```bash
# Offline model tests
cargo test

# Live API tests (require network access)
cargo test --test live -- --ignored
```

## Questions, Issues, Feedback, Contributions

Whether you want to make any bug reports, feature requests, or contribute to the wrapper, simply open an issue or pull request in this repository.
If GitHub is not your type, you can find me on [Discord](https://discord.com/invite/b22kMKuwbS), my username is @seria_ati.
