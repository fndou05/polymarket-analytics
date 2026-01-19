//! HTTP server wiring for the Polymarket analytics backend.

mod models;
mod gamma;
mod transform;

use axum::{
    extract::{Path, Query},
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use gamma::fetch_raw_markets_from_events;
use models::{AnalyzedMarket, MarketStatus};
use serde::Deserialize;
use std::sync::Arc;
use reqwest::Client;

/// Basic health check for uptime probes.
async fn health() -> &'static str {
    "ok"
}

/// Returns analyzed markets filtered by optional status query.
async fn get_markets(
    Query(params): Query<MarketsQuery>,
    State(client): State<Arc<Client>>,
) -> Result<Json<Vec<AnalyzedMarket>>, StatusCode> {
    // Apply status filtering after transformation to keep the API contract stable.
    let filter = parse_status_filter(params.status);

    let raw = fetch_raw_markets_from_events(&client)
        .await
        .map_err(|e| {
            eprintln!("fetch_markets failed: {:?}", e);
            StatusCode::BAD_GATEWAY
        })?;

    let analyzed: Vec<AnalyzedMarket> = raw
        .into_iter()
        .filter_map(transform::analyze_market)
        .filter(|m| filter.matches(m.status))
        .collect();

    Ok(Json(analyzed))
}

/// Fetch a single analyzed market by id (if present in current ingestion).
async fn get_market_by_id(
    Path(id): Path<String>,
    State(client): State<Arc<Client>>,
) -> Result<Json<AnalyzedMarket>, StatusCode> {
    let raw = fetch_raw_markets_from_events(&client)
        .await
        .map_err(|e| {
            eprintln!("fetch_markets failed: {:?}", e);
            StatusCode::BAD_GATEWAY
        })?;

    raw.into_iter()
        .filter_map(transform::analyze_market)
        .find(|m| m.id == id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Server entry point.
#[tokio::main]
async fn main() {
    let client = Arc::new(Client::new());

    let app = Router::new()
        .route("/health", get(health))
        .route("/markets", get(get_markets))
        .route("/markets/:id", get(get_market_by_id))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct MarketsQuery {
    status: Option<String>,
}

enum StatusFilter {
    Active,
    Closed,
    Resolved,
    Inactive,
    All,
}

impl StatusFilter {
    fn matches(&self, status: MarketStatus) -> bool {
        match self {
            StatusFilter::Active => status == MarketStatus::Active,
            StatusFilter::Closed => status == MarketStatus::Closed,
            StatusFilter::Resolved => status == MarketStatus::Resolved,
            StatusFilter::Inactive => {
                status == MarketStatus::Closed || status == MarketStatus::Resolved
            }
            StatusFilter::All => true,
        }
    }
}

fn parse_status_filter(value: Option<String>) -> StatusFilter {
    match value.as_deref().map(|v| v.trim().to_ascii_lowercase()) {
        Some(ref v) if v == "closed" => StatusFilter::Closed,
        Some(ref v) if v == "resolved" => StatusFilter::Resolved,
        Some(ref v) if v == "inactive" => StatusFilter::Inactive,
        Some(ref v) if v == "all" => StatusFilter::All,
        _ => StatusFilter::Active,
    }
}
