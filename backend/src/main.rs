//! HTTP server wiring for the Polymarket analytics backend.

mod models;

use axum::{routing::get, Json, Router};
use models::{
    AnalyzedMarket, MarketContext, MarketTiming, MarketType, Outcome,
    PricingInfo, ResolutionInfo,
};

/// Health check endpoint used for liveness monitoring.
async fn health() -> &'static str {
    "ok"
}

/// Temporary stub endpoint returning normalized market data.
///
/// This will later:
/// - Fetch Gamma API data
/// - Parse into RawMarket structs
/// - Transform into AnalyzedMarket
/// - Cache results in Redis
async fn get_markets() -> Json<Vec<AnalyzedMarket>> {
    let markets = vec![
        AnalyzedMarket {
            id: "btc-100k-2025".to_string(),
            question: "Will Bitcoin hit $100k by the end of 2025?".to_string(),
            description: Some("Prediction on long-term BTC price performance".to_string()),
            category: Some("Crypto".to_string()),
            market_type: MarketType::Binary,
            outcomes: vec![
                Outcome {
                    label: "Yes".to_string(),
                    probability: Some(0.62),
                },
                Outcome {
                    label: "No".to_string(),
                    probability: Some(0.38),
                },
            ],
            pricing: PricingInfo {
                best_bid: Some(0.61),
                best_ask: Some(0.63),
                mid_price: Some(0.62),
            },
            volume_usd: 150_000.0,
            liquidity_usd: 25_000.0,
            timing: MarketTiming {
                end_time: Some("2025-12-31T23:59:59Z".to_string()),
                closed_time: None,
                is_closed: false,
            },
            resolution: ResolutionInfo {
                resolution_source: Some("Market oracle".to_string()),
                is_resolved: false,
            },
            context: MarketContext {
                event_title: Some("Bitcoin price milestones".to_string()),
                tags: vec!["crypto".to_string(), "btc".to_string()],
            },
        },
    ];

    Json(markets)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/markets", get(get_markets));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port");

    println!("Server running at http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
