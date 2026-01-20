# Market Model

This document defines the backend’s **analyzed market** representation.

---

## Design Goals

- Normalize inconsistent upstream data
- Support all Polymarket market types
- Provide frontend- and LLM-friendly fields
- Avoid leaking Gamma schema quirks

---

## AnalyzedMarket (v2)

Each market includes:

- `id`
- `external_slug`
- `question`
- `description`
- `category`
- `market_type`
- `outcomes`
- `pricing`
- `volume_usd`
- `liquidity_usd`
- `timing`
- `status`
- `resolution`
- `context`
- **derived analytics fields** (see below)

---

## Market Status

Markets are explicitly classified as:

- `Active`
- `Closed`
- `Resolved`

Status is derived from raw Gamma fields rather than inferred at the UI layer.
This ensures consistency across consumers.

---

## Outcomes

Outcomes are normalized into a consistent structure:

- Label
- Price (if available)
- Probability (binary markets only)

Outcome ordering is preserved from the upstream source.

---

## Derived Analytics Fields

In addition to normalized raw data, each market includes **deterministic analytics fields**
computed at transformation time.

Examples include:

- Confidence or signal heuristics
- Liquidity-based rankings
- Entropy or dispersion metrics

These fields are:
- Deterministic
- Reproducible
- Not AI-generated
- Computed without external state

This allows downstream consumers to reason about markets without reimplementing logic.

---

## Why Normalize?

Gamma data:
- Uses mixed field types
- Omits fields inconsistently
- Changes shape across endpoints

Normalization provides:
- Stable API contracts
- Predictable frontend rendering
- Easier analytics and reasoning

---

## Collection Shape & Pagination

Market endpoints return markets as a **paginated collection**, not as an unbounded list.

Pagination is cursor-based and deterministic to support:
- Frontend incremental loading
- LLM chunking and summarization
- Future historical storage

### Request Parameters

- `limit` (optional): maximum number of markets to return
- `cursor` (optional): opaque cursor identifying where to continue
- `status` (optional): market status filter

Example:

```
GET /markets?status=active&limit=50&cursor=abc1
```

### Response Shape

```
{
"data": [AnalyzedMarket, ...],
"next_cursor": "abc123",
"has_more": true
}
```


### Ordering Guarantees

- Markets are ordered deterministically (e.g. by id or volume)
- Cursor pagination avoids offset-based instability
- Re-fetching with the same cursor is safe and repeatable

Pagination does not alter the `AnalyzedMarket` schema itself.