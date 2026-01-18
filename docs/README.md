# Polymarket Analytics Platform

A full-stack analytics platform built on top of Polymarket’s public APIs.

This project focuses on identifying high-signal prediction markets, analyzing market behavior and sentiment, and laying the foundation for future AI-assisted research tools.

This is an **analytics-first product**, not a trading platform.

---

## Project Goals

- Surface high-signal Polymarket markets
- Normalize complex external data into a stable internal schema
- Provide fast, clean REST APIs for analytics consumption
- Support future AI-driven market analysis
- Keep the MVP scope intentionally small and realistic

---

## MVP Scope

### Included

- Read-only market analytics
- Market discovery and filtering
- Volume- and activity-based prioritization
- Cached API responses for performance
- REST API consumed by a Next.js frontend

### Explicitly Not Included

- User accounts or authentication
- Trading, order placement, or wallets
- Real-time WebSocket streaming
- AI-generated recommendations
- Long-term historical storage

---

## Tech Stack

### Backend

- Rust
- Axum (HTTP server)
- Tokio (async runtime)
- Reqwest (HTTP client)
- Redis (cache only)
- Serde (JSON serialization)

### Frontend

- Next.js
- React
- TypeScript
- Tailwind CSS

---

## Repository Structure

```

polymarket-analytics/
├── backend/     # Rust backend service
├── frontend/    # Next.js frontend application
├── docs/        # Architecture, data flow, and design decisions
└── README.md

````

---

## Running the Backend Locally

From the repository root:

```bash
cd backend
cargo run
````

The server will start at:

```
http://localhost:3000
```

### Available Endpoints

* `GET /health`
  Service liveness check

* `GET /markets`
  List of markets (mock data initially)

---

## Project Status

This project is under active development.

### Current Focus

* Backend data ingestion and normalization
* Clean and stable API contracts
* Frontend analytics UI using mock data

AI features and advanced analytics are intentionally deferred until the data layer stabilizes.