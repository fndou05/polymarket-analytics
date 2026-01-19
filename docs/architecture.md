# Backend Architecture

The backend is a **read-only analytics service** that sits between Polymarket’s
public Gamma APIs and downstream consumers (frontend, analytics tools, LLMs).

It is not a trading system and does not place orders.

---

## High-Level Architecture (v1)

[ Polymarket Gamma API ]
|
v
[ Rust Backend (Axum) ]

Event-based ingestion

Raw → analyzed normalization

Derived market status and analytics
|
v
[ REST API ]

GET /health

GET /markets

GET /markets/:id
|
v
[ Frontend / Analytics / LLMs ]

Filtering

Visualization

Reasoning

---

## Core Responsibilities

### The backend DOES

- Fetch public Polymarket data via Gamma
- Treat **events as the ingestion source of truth**
- Flatten event markets into a unified stream
- Normalize inconsistent external schemas
- Derive explicit market state (active, closed, resolved)
- Expose a clean, stable REST API for analytics consumption

### The backend DOES NOT

- Place trades or manage orders
- Manage users or authentication
- Store long-term historical data
- Act as a system of record
- Perform real-time streaming (v1)

---

## Design Philosophy

- Correctness over convenience
- Normalize early, expose stable contracts
- Accept upstream inconsistency, never leak it
- Defer infrastructure until value is proven

---

## Planned Extensions (Post-v1)

- In-memory TTL caching (v1.1)
- Redis-backed cache (v2)
- Background refresh tasks
- Pagination
- Historical snapshots
