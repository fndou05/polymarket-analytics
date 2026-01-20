//! HTTP server wiring for the Polymarket analytics backend.

mod analytics;
mod cache;
mod config;
mod error;
mod gamma;
mod models;
mod pagination;
mod transform;

use axum::{
    extract::{FromRef, State},
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use cache::redis::RedisCacheStore;
use cache::{CacheStore, CacheStoreImpl, InMemoryCacheStore};
use config::{CacheBackend, CacheConfig};
use error::ApiError;
use gamma::fetch_raw_markets_from_events;
use models::{AnalyzedMarket, MarketStatus, PaginatedResponse};
use pagination::{paginate_markets, parse_status_filter, StatusFilter};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;

/// Basic health check for uptime probes.
async fn health() -> &'static str {
    "ok"
}

/// Returns analyzed markets filtered by optional status query.
async fn get_markets(
    Query(params): Query<MarketsQuery>,
    State(client): State<Arc<Client>>,
    State(cache): State<Arc<CacheStoreImpl>>,
    State(cache_ttl): State<Duration>,
) -> Result<Json<PaginatedResponse<AnalyzedMarket>>, ApiError> {
    // Apply status filtering after transformation to keep the API contract stable.
    let filter = parse_status_filter(params.status);

    let start = std::time::Instant::now();

    let limit = params.limit.unwrap_or(50).min(100);
    let cursor = params.cursor.as_deref();

    if matches!(filter, StatusFilter::Active) {
        match cache.get::<Vec<AnalyzedMarket>>(CACHE_KEY_ACTIVE).await {
            Ok(Some(markets)) => {
                tracing::info!("cache hit: active");
                let response = paginate_markets(markets, &filter, limit, cursor);
                let elapsed = start.elapsed();
                tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
                return Ok(Json(response));
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!("cache read failed: {}", err);
            }
        }

        match cache.get::<Vec<AnalyzedMarket>>(CACHE_KEY_ALL).await {
            Ok(Some(markets)) => {
                tracing::info!("cache hit: all");
                let response = paginate_markets(markets, &filter, limit, cursor);
                let elapsed = start.elapsed();
                tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
                return Ok(Json(response));
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!("cache read failed: {}", err);
            }
        }
    } else {
        match cache.get::<Vec<AnalyzedMarket>>(CACHE_KEY_ALL).await {
            Ok(Some(markets)) => {
                tracing::info!("cache hit: all");
                let response = paginate_markets(markets, &filter, limit, cursor);
                let elapsed = start.elapsed();
                tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
                return Ok(Json(response));
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!("cache read failed: {}", err);
            }
        }
    }

    tracing::info!("cache miss: markets");

    let raw = fetch_raw_markets_from_events(&client).await.map_err(|e| {
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

    if let Err(err) = cache.set(CACHE_KEY_ALL, &analyzed_all, cache_ttl).await {
        tracing::error!("cache write failed: {}", err);
    }

    if let Err(err) = cache
        .set(CACHE_KEY_ACTIVE, &analyzed_active, cache_ttl)
        .await
    {
        tracing::error!("cache write failed: {}", err);
    }

    let response = if matches!(filter, StatusFilter::Active) {
        Ok(Json(paginate_markets(
            analyzed_active,
            &filter,
            limit,
            cursor,
        )))
    } else {
        Ok(Json(paginate_markets(analyzed_all, &filter, limit, cursor)))
    };

    let elapsed = start.elapsed();
    tracing::info!("GET /markets status={:?} duration={:?}", filter, elapsed);
    response
}

/// Fetch a single analyzed market by id (if present in current ingestion).
async fn get_market_by_id(
    Path(id): Path<String>,
    State(client): State<Arc<Client>>,
    State(cache): State<Arc<CacheStoreImpl>>,
    State(cache_ttl): State<Duration>,
) -> Result<Json<AnalyzedMarket>, ApiError> {
    let start = std::time::Instant::now();

    match cache.get::<Vec<AnalyzedMarket>>(CACHE_KEY_ALL).await {
        Ok(Some(markets)) => {
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
        Ok(None) => {}
        Err(err) => {
            tracing::error!("cache read failed: {}", err);
        }
    }

    match cache.get::<Vec<AnalyzedMarket>>(CACHE_KEY_ACTIVE).await {
        Ok(Some(markets)) => {
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
        Ok(None) => {}
        Err(err) => {
            tracing::error!("cache read failed: {}", err);
        }
    }

    tracing::info!("cache miss: market by id");

    let raw = fetch_raw_markets_from_events(&client).await.map_err(|e| {
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

    if let Err(err) = cache.set(CACHE_KEY_ALL, &analyzed_all, cache_ttl).await {
        tracing::error!("cache write failed: {}", err);
    }

    if let Err(err) = cache
        .set(CACHE_KEY_ACTIVE, &analyzed_active, cache_ttl)
        .await
    {
        tracing::error!("cache write failed: {}", err);
    }

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
    let cache_config = CacheConfig::from_env();
    let backend_name = match &cache_config.backend {
        CacheBackend::InMemory => "in-memory",
        CacheBackend::Redis { .. } => "redis",
    };
    tracing::info!("cache backend: {} ttl={:?}", backend_name, cache_config.ttl);
    let cache = match cache_config.backend {
        CacheBackend::InMemory => CacheStoreImpl::InMemory(InMemoryCacheStore::new()),
        CacheBackend::Redis { ref url } => {
            CacheStoreImpl::Redis(RedisCacheStore::new(url).expect("failed to connect to redis"))
        }
    };

    let app_state = AppState {
        client,
        cache: Arc::new(cache),
        cache_ttl: cache_config.ttl,
    };

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
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
    cache: Arc<CacheStoreImpl>,
    cache_ttl: Duration,
}

impl FromRef<AppState> for Arc<Client> {
    fn from_ref(state: &AppState) -> Arc<Client> {
        state.client.clone()
    }
}

impl FromRef<AppState> for Arc<CacheStoreImpl> {
    fn from_ref(state: &AppState) -> Arc<CacheStoreImpl> {
        state.cache.clone()
    }
}

impl FromRef<AppState> for Duration {
    fn from_ref(state: &AppState) -> Duration {
        state.cache_ttl
    }
}

const CACHE_KEY_ACTIVE: &str = "markets:active";
const CACHE_KEY_ALL: &str = "markets:all";
