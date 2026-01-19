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
- Not a real-time streaming service (v1)

---

## Architecture Overview

- Event-based ingestion from Gamma
- Defensive raw deserialization
- Raw → analyzed market transformation
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

---

## API Endpoints

- `GET /health`
- `GET /markets`
- `GET /markets?status=active|closed|resolved|inactive|all`
- `GET /markets/:id`

---

## Running Locally

```
cargo run

Server runs at:
http://localhost:3000
```

## Project Status

v1 complete

Current focus:
- Stable ingestion
- Clean API contracts
- Analytics-ready data

Next steps:
- In-memory caching
- Improved error handling
- Documentation polish