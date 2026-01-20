# Engineering Decisions

This document records intentional architectural decisions and tradeoffs
made during development of the Polymarket Analytics backend.

---

## Rust for Backend

Chosen for:
- Strong type safety
- Async-first ecosystem
- Performance predictability
- Explicit error handling
- Resume credibility

Rust enables the backend to behave like a long-running service
rather than a scripting layer.

---

## Axum for HTTP Layer

Chosen for:
- Modern async design
- Explicit extractor-based request handling
- Clear separation of request parsing and business logic
- Minimal abstraction overhead

---

## Event-Based Ingestion

Markets are ingested by flattening **events**, not by querying markets directly.

Reasons:
- Active markets reliably exist under events
- Event grouping reflects Polymarket’s domain model
- Standalone market endpoints are inconsistent

Gamma remains the sole source of truth.

---

## Defensive Deserialization

Gamma payloads are treated as untrusted input.

Decisions:
- Tolerate mixed field types
- Allow missing or null fields
- Avoid failing ingestion due to partial data

This prevents upstream schema instability from breaking the service.

---

## Read-Through Caching

- Cache stores analyzed data only
- Cache is never authoritative
- TTL-based freshness
- Read-through semantics
- Redis and in-memory caches share identical behavior

Caching exists to reduce load on Gamma and improve response latency,
not to own data.

---

## Cursor-Based Pagination

Chosen over offset-based pagination because:
- Stable across refreshes
- Safer for changing datasets
- Deterministic ordering
- Better support for frontend incremental loading
- Better support for LLM chunking

---

## Deterministic Analytics

Analytics fields are:
- Derived deterministically
- Computed synchronously
- Based only on current market data

No AI or probabilistic inference is used at this stage.

---

## No Database (Current Phase)

Deferred because:
- No user-owned data
- No write operations
- Gamma remains the source of truth
- Avoids premature infrastructure complexity

Persistent storage is planned only when historical data becomes necessary.