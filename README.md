# Polymarket Analytics API

A read-only backend written in Rust that pulls active events from Polymarket's public Gamma API, normalizes the market data, computes a few simple analytics, and serves the results through a paginated REST API. It does not place trades, store data, or manage users.

> Most of the code in this repo was written with AI. I made the design decisions, then read through, ran, and tested what it produced.

## Quick start

```
cargo run
```

The server starts on `http://localhost:3000`. To try it:

```
curl "http://localhost:3000/markets?status=active&limit=5"
```

Run the tests with `cargo test`.

Configuration is through two optional environment variables:

| Variable | Effect |
| --- | --- |
| `REDIS_URL` | Use Redis for the cache. If unset, an in-memory cache is used. |
| `CACHE_TTL_SECONDS` | How long cached data lives. Defaults to 30. |

## API

| Endpoint | Description |
| --- | --- |
| `GET /health` | Returns `ok`. |
| `GET /markets` | Paginated list of markets. Accepts `status` (`active`, `closed`, `resolved`, `inactive`, `all`; default `active`), `limit` (default 50, max 100), and `cursor`. |
| `GET /markets/:id` | A single market by id. |

`/markets` responds with this shape:

```json
{ "data": [ ... ], "next_cursor": "...", "has_more": true }
```

Each market is returned in a normalized form (id, slug, question, outcomes with prices, volume, liquidity, timing, status) along with four analytics fields:

- `implied_prob_spread`: the gap between the two outcome probabilities
- `outcome_entropy`: Shannon entropy of the outcome probabilities
- `binary_confidence_score`: the higher of the two probabilities
- `is_high_signal`: true when the confidence score is at least 0.8 and liquidity is at least $10,000

Probabilities are only derived for binary markets, so these fields are `null` for categorical and scalar markets.

Status is derived from the raw data: a closed market with a resolution source is `resolved`, a closed market without one is `closed`, and anything else is `active`.

## How it works

```
Gamma API
  -> fetch active events and flatten out their markets
  -> parse the raw JSON into tolerant raw models
  -> transform into normalized markets and compute analytics
  -> cache the result (in-memory or Redis)
  -> filter by status, sort, and paginate with a cursor
  -> JSON response
```

Parsing, transformation, and analytics run on a cache miss. Filtering, sorting, and pagination run on every request against the cached list.

## Design decisions

| Decision | Reason |
| --- | --- |
| Fetch events, not markets | Active markets appear reliably nested under events, while the standalone market endpoints were less consistent. |
| Tolerant parsing | Gamma's types are inconsistent: outcomes and prices can arrive as an array or as a string containing JSON, prices can be numbers or numeric strings, and some fields are empty or missing. The raw models accept all of these. A missing market type defaults to binary, and unrecognized types are skipped. |
| Cache behind a trait | `CacheStore` has in-memory and Redis implementations, chosen by `REDIS_URL`. It only holds processed market lists and is never the source of truth. Gamma is. |
| Cursor pagination | The cursor is the id of the last market returned, and results are sorted by volume (highest first), then id. Offsets would shift as the data changes between refreshes. |
| No database | There is no user data and nothing is written, so persistence can wait until historical data is needed. |
| Rust and Axum | I wanted to learn Rust, and there is plenty of documentation and community client libraries for Polymarket's APIs in Rust. |

## Testing

`cargo test` covers:

- **Cache:** values are returned before the TTL expires and missing after, and a read-through pattern fetches only once per TTL window (in-memory backend only)
- **Pagination:** ordering is stable, pages don't overlap, the cursor resumes where the previous page ended, and `has_more` becomes false at the end
- **Transformation:** binary markets normalize correctly, status is derived correctly, a missing market type defaults to binary, and empty strings don't cause errors

The Redis backend, the HTTP handlers, the analytics functions, and live calls to Gamma are not tested.

## Known limitations

- Only active events are fetched (up to 200), so the `closed` and `resolved` filters only return markets that sit inside currently active events.
- A single malformed market makes the whole fetch fail with a 502. Parsing tolerates the type differences above but does not isolate bad markets.
- If the market a cursor points to disappears after a cache refresh, pagination silently restarts from the first page.
- The Redis backend is untested.
- There is no historical storage. `MarketSnapshot` in the models is only a design sketch.

## Background

A friend who was exploring a startup idea asked if I could build a backend that pulls from the Gamma API. I took it on to learn Rust and to understand how backends and APIs work, since I had never built one.

As noted above, AI wrote most of the code, and I wrote very little of it myself. My part was deciding what the service would do and how it was structured, then verifying it: I read through the code, ran the tests, and checked the endpoints with curl. This isn't how I usually work. I normally learn a new language through online courses and a textbook, work through problems on my own, and use AI mainly as a tutor for questions and for checking my reasoning. On this project, a deadline and a fixed feature set meant I relied on it for the code itself.

The hardest part was Rust, which is very different from C++ and Java. It took real effort to understand the code well enough to be comfortable pushing it. Backend work was also new compared with my coursework. With no database to design, most of what I learned was the pipeline: pulling in outside data, normalizing it, caching it with Redis, paginating it, and exposing it through endpoints, along with writing tests.

I came away understanding what building a backend involves, though with less experience writing one from scratch. If I did it again, I would take more time, write most of it by hand, and make sure I understood each part before moving on.

## What's next

- Historical snapshots and time-series analytics
- A frontend
