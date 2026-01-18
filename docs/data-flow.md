# Data Flow

This document describes how market data flows through the system, from Polymarket’s public APIs to frontend analytics views.

The backend acts as a controlled ingestion and transformation layer, not a passive proxy.

---

## High-Level Flow

1. Fetch raw market data from Polymarket Gamma API
2. Apply ingestion controls (rate limiting, retries, scheduling)
3. Normalize external schemas into internal models
4. Filter markets based on relevance and activity
5. Enrich markets with computed analytics
6. Cache processed snapshots in Redis
7. Serve data through read-only REST endpoints
8. Visualize and analyze data in the frontend

---

## Detailed Pipeline

### 1. Data Ingestion

The Rust backend pulls data from the Polymarket Gamma API, including:

- Markets
- Events
- Categories
- Volume and liquidity metadata

Ingestion is protected by:
- Rate limiting
- Retry logic
- Scheduled polling

This ensures API safety and predictable load.

---

### 2. Schema Normalization

Raw API responses are parsed and converted into stable internal models.

Normalization includes:
- Converting external field names into internal formats
- Assigning stable internal IDs
- Cleaning and standardizing timestamps
- Resolving inconsistent or optional fields

After this step, downstream components never interact with raw API schemas.

---

### 3. Market Filtering

Normalized markets are filtered to remove low-signal or irrelevant data.

Examples include:
- Excluding inactive or resolved markets
- Enforcing minimum volume or liquidity thresholds
- Selecting specific categories or event types

This keeps the dataset focused and frontend-friendly.

---

### 4. Market Enrichment

Filtered markets are enriched with computed fields such as:
- Aggregated volume or activity metrics
- Derived statistics
- Frontend-ready fields

This avoids pushing analytics logic into the UI layer.

---

### 5. Caching Layer (Redis)

Processed market snapshots are stored in Redis as short-lived cache entries.

Redis stores:
- Cached market lists
- Cached per-market detail views
- TTL-based freshness metadata

Redis is never treated as a source of truth and can be safely flushed or rebuilt.

---

### 6. API Access

The backend exposes read-only REST endpoints via Axum:

- `GET /health`
- `GET /markets`
- `GET /markets/:id`

Requests are served from cache when possible, with background refreshes when data expires.

---

### 7. Frontend Consumption

The Next.js frontend consumes backend APIs to provide:

- Market discovery views
- Market detail pages
- Charts and trend analysis
- Filtering and sorting controls

The frontend never interacts directly with Polymarket APIs.

---

## Why This Architecture Exists

Polymarket’s APIs are:
- Rich but complex
- Not optimized for frontend consumption
- Subject to strict rate limits

This backend acts as a **buffer, translator, and analytics preprocessor**, enabling fast and stable data access.

---

## Refresh Strategy (MVP)

- Fixed-interval polling
- Cache-first reads
- TTL-based freshness
- No WebSockets
- No push-based updates

---

## Future (Non-MVP)

- Historical snapshot storage
- Event-driven ingestion
- AI feature pipelines
- Real-time streaming and alerts
