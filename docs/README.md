# Polymarket Analytics Backend

A Rust-based analytics backend that consumes Polymarket’s public Gamma APIs
and exposes a clean, normalized market analytics API.

This is an analytics-first system, not a trading platform.

---

## Project Goals

- Normalize messy real-world API data into stable contracts
- Provide clean REST APIs for analytics and visualization
- Support future AI/LLM reasoning over market data
- Keep architecture layered, explicit, and extensible

---

## What This Is Not

- Not a trading backend
- Not a Polymarket clone
- Not a system of record
- Not a real-time streaming service

---

## Architecture Overview

- Event-based ingestion from Gamma
- Defensive raw deserialization
- Raw → analyzed market transformation
- Deterministic analytics derivation
- Read-through TTL caching (in-memory or Redis)
- Cursor-based pagination
- Request-time filtering
- Read-only REST API

---

## Tech Stack

**Backend**
- Rust
- Axum
- Tokio
- Reqwest
- Serde
- Redis (optional, cache backend)
- Tracing (structured logs)

---

## API Endpoints

- `GET /health`
- `GET /markets`
- `GET /markets?status=active|closed|resolved|inactive|all&limit=&cursor=`
- `GET /markets/:id`

Responses are paginated and include metadata for incremental consumption.

---

## Running Locally

```
cargo run
Server runs at: http://localhost:3000
```


---

## Project Status

Backend stable (v2).

Current capabilities:
- Stable event-based ingestion
- Explicit market normalization
- Deterministic analytics fields
- Cursor-based pagination
- Pluggable caching (in-memory / Redis)
- Explicit error handling and tracing

Next planned work:
- Historical snapshot storage
- Time-series analytics
- Frontend integration