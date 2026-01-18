# Engineering Decisions

This document records intentional design choices.

---

## Rust for Backend

Chosen for:
- Performance
- Safety
- Async support
- Resume credibility
- Alignment with Polymarket ecosystem

---

## Axum for HTTP

Chosen for:
- Modern async design
- Type safety
- Clean routing
- Minimal boilerplate

---

## Redis as Cache

Chosen for:
- Speed
- Simplicity
- Stateless rebuilds
- Avoiding database complexity in MVP

---

## REST over WebSockets

Chosen for MVP because:
- Easier to debug
- Easier to cache
- Easier to reason about
- Enough for analytics use cases

---

## No Database in MVP

Reason:
- No user data
- No ownership of market data
- Redis cache is sufficient
- Reduces operational overhead

---

## AI Deferred

AI analysis is intentionally postponed until:
- Market data schema stabilizes
- Usage patterns are understood
- Clear value signals emerge
