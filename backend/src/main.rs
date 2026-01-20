//! HTTP server wiring for the Polymarket analytics backend.

mod models;
mod gamma;
mod transform;
mod error;
mod cache;

use axum::{
    extract::{Path, Query},
    extract::{FromRef, State},
    routing::get,
    Json, Router,
};
use cache::{CacheStore, InMemoryCacheStore};
use error::ApiError;
use gamma::fetch_raw_markets_from_events;
use models::{AnalyzedMarket, MarketStatus};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use reqwest::Client;

/// Basic health check for uptime probes.
async fn health() -> &'static str {
    "ok"
}

/// Returns analyzed markets filtered by optional status query.
async fn get_markets(
    Query(params): Query<MarketsQuery>,
    State(client): State<Arc<Client>>,
    State(cache): State<Arc<dyn CacheStore>>,
) -> Result<Json<Vec<AnalyzedMarket>>, ApiError> {
    // Apply status filtering after transformation to keep the API contract stable.
    let filter = parse_status_filter(params.status);

    let start = std::time::Instant::now();

    if matches!(filter, StatusFilter::Active) {
        if let Some(markets) = cache.get_active() {
            tracing::info!("cache hit: active");
            let elapsed = start.elapsed();
            tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
            return Ok(Json(markets));
        }
        if let Some(markets) = cache.get_all() {
            tracing::info!("cache hit: all");
            let filtered = apply_filter(&filter, markets);
            let elapsed = start.elapsed();
            tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
            return Ok(Json(filtered));
        }
    } else if let Some(markets) = cache.get_all() {
        tracing::info!("cache hit: all");
        let filtered = apply_filter(&filter, markets);
        let elapsed = start.elapsed();
        tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
        return Ok(Json(filtered));
    }

    tracing::info!("cache miss: markets");

    let raw = fetch_raw_markets_from_events(&client)
        .await
        .map_err(|e| {
            let api_error = if e.is_decode() {
                ApiError::GammaDeserialize(e)
            } else {
                ApiError::GammaFetch(e)
            };
            tracing::error!("request failed: {}", api_error);
            api_error
        })?;

    let analyzed_all: Vec<AnalyzedMarket> = raw
        .into_iter()
        .filter_map(transform::analyze_market)
        .collect();

    let analyzed_active: Vec<AnalyzedMarket> = analyzed_all
        .iter()
        .filter(|m| m.status == MarketStatus::Active)
        .cloned()
        .collect();

    cache.set_all(analyzed_all.clone());
    cache.set_active(analyzed_active.clone());

    let response = if matches!(filter, StatusFilter::Active) {
        Ok(Json(analyzed_active))
    } else {
        let filtered = apply_filter(&filter, analyzed_all);
        Ok(Json(filtered))
    };

    let elapsed = start.elapsed();
    tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
    response
}

/// Fetch a single analyzed market by id (if present in current ingestion).
async fn get_market_by_id(
    Path(id): Path<String>,
    State(client): State<Arc<Client>>,
    State(cache): State<Arc<dyn CacheStore>>,
) -> Result<Json<AnalyzedMarket>, ApiError> {
    let start = std::time::Instant::now();

    if let Some(markets) = cache.get_all() {
        tracing::info!("cache hit: all");
        let response = markets
            .into_iter()
            .find(|m| m.id == id)
            .map(Json)
            .ok_or(ApiError::NotFound);
        let elapsed = start.elapsed();
        tracing::info!("GET /markets/:id duration={:?}", elapsed);
        return response;
    }

    if let Some(markets) = cache.get_active() {
        tracing::info!("cache hit: active");
        let response = markets
            .into_iter()
            .find(|m| m.id == id)
            .map(Json)
            .ok_or(ApiError::NotFound);
        let elapsed = start.elapsed();
        tracing::info!("GET /markets/:id duration={:?}", elapsed);
        return response;
    }

    tracing::info!("cache miss: market by id");

    let raw = fetch_raw_markets_from_events(&client)
        .await
        .map_err(|e| {
            let api_error = if e.is_decode() {
                ApiError::GammaDeserialize(e)
            } else {
                ApiError::GammaFetch(e)
            };
            tracing::error!("request failed: {}", api_error);
            api_error
        })?;

    let analyzed_all: Vec<AnalyzedMarket> = raw
        .into_iter()
        .filter_map(transform::analyze_market)
        .collect();

    let analyzed_active: Vec<AnalyzedMarket> = analyzed_all
        .iter()
        .filter(|m| m.status == MarketStatus::Active)
        .cloned()
        .collect();

    cache.set_all(analyzed_all.clone());
    cache.set_active(analyzed_active);

    let response = analyzed_all
        .into_iter()
        .find(|m| m.id == id)
        .map(Json)
        .ok_or(ApiError::NotFound);

    let elapsed = start.elapsed();
    tracing::info!("GET /markets/:id duration={:?}", elapsed);
    response
}

/// Server entry point.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let client = Arc::new(Client::new());
    // TTL = 45 seconds to keep Gamma load low without going stale for long.
    let cache: Arc<dyn CacheStore> =
        Arc::new(InMemoryCacheStore::new(Duration::from_secs(45)));

    let app_state = AppState { client, cache };

    let app = Router::new()
        .route("/health", get(health))
        .route("/markets", get(get_markets))
        .route("/markets/:id", get(get_market_by_id))
        .with_state(app_state);

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

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
    cache: Arc<dyn CacheStore>,
}

impl FromRef<AppState> for Arc<Client> {
    fn from_ref(state: &AppState) -> Arc<Client> {
        state.client.clone()
    }
}

impl FromRef<AppState> for Arc<dyn CacheStore> {
    fn from_ref(state: &AppState) -> Arc<dyn CacheStore> {
        state.cache.clone()
    }
}

#[derive(Debug)]
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

fn apply_filter(
    filter: &StatusFilter,
    markets: Vec<AnalyzedMarket>,
) -> Vec<AnalyzedMarket> {
    match filter {
        StatusFilter::All => markets,
        _ => markets
            .into_iter()
            .filter(|m| filter.matches(m.status))
            .collect(),
    }
}
