# Engineering Decisions

This document records intentional architectural decisions.

---

## Rust for Backend

Chosen for:
- Strong type safety
- Predictable performance
- Async-first ecosystem
- Resume credibility
- Alignment with Polymarket tooling

---

## Axum for HTTP Layer

Chosen for:
- Modern async design
- Explicit extractor-based request handling
- Minimal abstraction overhead
- Clear separation of concerns

---

## Event-Based Ingestion

Markets are ingested by flattening **events**, not by querying markets directly.

Reasoning:
- Active markets are reliably nested under events
- Standalone market endpoints are inconsistent
- Event grouping reflects Polymarket’s domain model

---

## Caching Strategy (v1)

- Source of truth: Polymarket Gamma API
- Cached data: AnalyzedMarket only
- Cache type: read-through TTL cache
- TTL: 60 seconds
- Stale data allowed within TTL
- Cache is never authoritative
- Redis will replace in-memory cache without API changes

---

## REST over WebSockets

Chosen for v1 because:
- Easier to debug
- Easier to cache
- Easier to reason about
- Sufficient for analytics use cases

---

## No Database in v1

Reasoning:
- No user-owned data
- No write operations
- Gamma remains source of truth
- Reduces operational overhead

---

## AI Deferred

AI analysis is deferred until:
- Market schema stabilizes
- Analytics needs are proven
- High-signal features are identified
