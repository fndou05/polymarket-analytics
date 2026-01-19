# Data Flow

This document describes how market data flows through the backend in **v1**.

The backend acts as an ingestion and normalization layer, not a passive proxy.

---

## High-Level Flow (v1)

1. Fetch active events from Polymarket Gamma API
2. Flatten event-contained markets
3. Deserialize raw market payloads defensively
4. Transform raw markets into analyzed markets
5. Apply request-time filtering
6. Serve results via read-only REST endpoints

---

## Detailed Pipeline

### 1. Ingestion (Gamma)

The backend fetches **events** from the Gamma API.

Events are treated as the authoritative source because:
- Active markets are reliably nested under events
- Standalone market endpoints are inconsistent
- Event grouping matches Polymarket’s domain model

---

### 2. Raw Deserialization

Gamma payloads are deserialized into internal raw models.

This layer:
- Tolerates missing or malformed fields
- Handles mixed string/number representations
- Never assumes schema stability

Raw models are internal only and never exposed.

---

### 3. Transformation

Raw markets are converted into analyzed markets.

Transformation includes:
- Normalizing market type (binary, categorical, scalar)
- Zipping outcomes with prices
- Deriving probabilities for binary markets
- Normalizing categories
- Deriving explicit market status

This layer is pure logic with no I/O.

---

### 4. API Filtering

Filtering is applied **after transformation** to keep the API contract stable.

Examples:
- Active vs closed markets
- Resolved vs unresolved markets

---

### 5. API Serving

The backend exposes a clean REST API:

- `GET /health`
- `GET /markets`
- `GET /markets/:id`

All responses contain analyzed, frontend-ready data.

---

## Future Flow (Post-v1)

- TTL-based in-memory caching
- Redis-backed cache
- Background refresh
- Historical snapshots
