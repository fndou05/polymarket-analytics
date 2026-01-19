# Market Model

This document defines the backend’s **analyzed market** representation.

---

## Design Goals

- Normalize inconsistent upstream data
- Support all Polymarket market types
- Provide frontend- and LLM-friendly fields
- Avoid leaking Gamma schema quirks

---

## AnalyzedMarket (v1)

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

---

## Market Status

Markets are explicitly classified as:

- `Active`
- `Closed`
- `Resolved`

Status is derived from raw Gamma fields rather than inferred at the UI layer.

---

## Outcomes

Outcomes are normalized into a consistent structure:

- Label
- Price (if available)
- Probability (binary markets only)

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

## Planned Extensions

- Market family inference
- Time interval inference
- Volatility metrics
- Historical snapshots
