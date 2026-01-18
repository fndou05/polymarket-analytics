# Market Model

This document defines the internal market representation.

---

## Design Goals

- Support all Polymarket market types
- Avoid assumptions about binary outcomes
- Preserve useful metadata
- Enable future AI analysis

---

## Core Market Fields (MVP)

- id
- question
- category
- volume
- status
- outcomes (flexible structure)

---

## Why Normalize?

Polymarket data:
- Varies by market type
- Contains nested structures
- Is not frontend-friendly

Normalization provides:
- Stable contracts
- Predictable frontend rendering
- Easier AI feature extraction

---

## Future Extensions

- Historical price snapshots
- Market volatility metrics
- Resolution probability trends
- Cross-market correlations
