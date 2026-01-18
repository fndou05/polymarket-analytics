# Polymarket Analytics

A market analytics platform built on top of Polymarket data.

## Architecture
- Rust backend that ingests and normalizes Polymarket Gamma API data
- Redis used as a cache for market snapshots
- REST API exposed to a Next.js frontend
- Frontend renders market discovery, filtering, and analytics

## Project Structure
- /backend — Rust Axum service
- /frontend — Next.js + TypeScript frontend
- /docs — architecture and API documentation

## Status
Early development (MVP phase).
