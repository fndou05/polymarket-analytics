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

- Event-based ingestion
- Defensive raw deserialization
- Raw → analyzed normalization
- Deterministic analytics derivation
- Read-through caching (in-memory or Redis)
- Cursor-based pagination
        |
        v
[ REST API ]

- GET /health
- GET /markets
- GET /markets/:id
        |
        v
[ Frontend / Analytics / LLMs ]

- Filtering
- Visualization
- Reasoning

---

## Core Responsibilities

### The backend DOES

- Fetch public Polymarket data via Gamma
- Treat **events as the ingestion source of truth**
- Flatten event markets into a unified stream
- Normalize inconsistent external schemas
- Derive explicit market state (active, closed, resolved)
- Compute deterministic analytics fields
- Cache analyzed data using read-through TTL semantics
- Expose a clean, stable REST API for analytics consumption

### The backend DOES NOT

- Place trades or manage orders
- Manage users or authentication
- Act as a system of record
- Assume real-time guarantees
- Perform write operations of any kind

---

## Caching Architecture (v1)

Caching is implemented as a **read-through layer**.

- Cache stores analyzed market collections only
- Cache is never authoritative
- TTL-based freshness
- Identical semantics across cache backends

The cache backend is selected at runtime:
- In-memory cache (default)
- Redis-backed cache (via environment configuration)

Switching cache backends does not change API behavior.

---

## Pagination Strategy (v1)

Market collections are paginated using **cursor-based pagination**.

Reasons:
- Stable across refreshes
- Safe for changing datasets
- Suitable for frontend incremental loading
- Suitable for LLM chunking

Ordering is deterministic and pagination is applied after caching.

---

## Design Philosophy

- Correctness over convenience
- Normalize early, expose stable contracts
- Accept upstream inconsistency, never leak it
- Prefer boring infrastructure that is easy to reason about
- Defer persistence until value is proven

---

## Planned Extensions (Post-v1)

- Background refresh tasks
- Historical snapshot storage
- Time-series analytics
- AI-driven interpretation layers