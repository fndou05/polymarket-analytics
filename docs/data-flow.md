# Data Flow

This document describes how market data flows through the backend in **v1**.

The backend acts as an ingestion, normalization, and analytics layer,
not a passive proxy.

---

## High-Level Flow (v1)

1. Fetch active events from Polymarket Gamma API
2. Flatten event-contained markets
3. Deserialize raw market payloads defensively
4. Transform raw markets into analyzed markets
5. Compute deterministic analytics fields
6. Cache analyzed results (read-through)
7. Apply request-time filtering and pagination
8. Serve results via read-only REST endpoints

---

## Detailed Pipeline

### 1. Ingestion (Gamma)

The backend fetches **events** from the Gamma API.

Events are treated as the authoritative source because:
- Active markets are reliably nested under events
- Standalone market endpoints are inconsistent
- Event grouping matches Polymarket’s domain model

Gamma remains the sole source of truth.

---

### 2. Raw Deserialization

Gamma payloads are deserialized into internal raw models.

This layer:
- Tolerates missing or malformed fields
- Handles mixed string/number representations
- Avoids failing ingestion due to partial data
- Never assumes schema stability

Raw models are internal only and never exposed.

---

### 3. Transformation

Raw markets are converted into analyzed markets.

Transformation includes:
- Normalizing market type (binary, categorical, scalar)
- Zipping outcomes with prices
- Deriving probabilities for binary markets
- Normalizing category strings
- Deriving explicit market status

This layer is pure logic with no HTTP or cache access.

---

### 4. Deterministic Analytics

Additional analytics fields are derived synchronously.

Examples:
- Probability spreads
- Confidence heuristics
- Outcome entropy
- Liquidity-based signals

These fields are:
- Deterministic
- Non-AI
- Computed at request time

---

### 5. Read-Through Caching

Analyzed market collections are cached using read-through semantics.

Caching behavior:
- Stores analyzed data only
- TTL-based freshness
- Cache is never authoritative
- Identical semantics across cache backends

In-memory and Redis caches are interchangeable.

---

### 6. Request-Time Filtering & Pagination

Filtering and pagination are applied **after caching**.

This ensures:
- Stable API contracts
- Deterministic pagination
- Safe incremental fetching

Pagination is cursor-based and ordering is deterministic.

---

### 7. API Serving

The backend exposes a clean, read-only REST API:

- `GET /health`
- `GET /markets`
- `GET /markets/:id`

All responses contain analyzed, analytics-ready data suitable
for frontend consumption and LLM processing.

---

## Non-Goals (v1)

- No background jobs
- No database
- No streaming or WebSockets
- No persistence guarantees

These are deferred intentionally.
