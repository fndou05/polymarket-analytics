# Backend Architecture

The backend is a **read-only analytics service** that sits between Polymarket’s public APIs and the frontend.

Its role is to ingest, normalize, cache, and expose market data in a form that is easy to analyze and visualize.

---

## High-Level Architecture

[ Polymarket Gamma API ]
|
v
[ Rust Backend (Ingestion Layer) ]

Fetch markets, events, categories

Rate limiting and retries

Schema normalization

Filtering and enrichment
|
v
[ Redis Cache ]

Cached market lists

Cached market detail

TTL-based freshness
|
v
[ REST API (Axum) ]

GET /health

GET /markets

GET /markets/:id
|
v
[ Next.js Frontend ]

Market discovery

Market detail views

Charts and filters

---

## Backend Responsibilities

### The backend DOES

- Fetch public Polymarket data
- Normalize inconsistent external schemas
- Filter markets by relevance, volume, and activity
- Cache market snapshots for fast access
- Provide stable REST APIs for frontend consumption

### The backend DOES NOT

- Place trades or route orders
- Manage users or authentication
- Act as a system of record
- Store long-term historical data
- Provide real-time streaming (MVP)

---

## Redis Usage

Redis is used strictly as a **cache**, not a database.

- Stores short-lived market snapshots
- Uses TTLs to ensure freshness
- Safe to flush or rebuild at any time
- Never treated as authoritative data

Polymarket’s public APIs remain the source of truth.

---

## Design Philosophy

- Clarity over cleverness
- Cache instead of persist
- Normalize early
- Scale only when necessary