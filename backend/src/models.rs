//! Domain models for the Polymarket analytics backend.
//!
//! IMPORTANT:
//! This file defines the *stable* internal representation used for
//! analytics, AI reasoning, and frontend consumption.
//!
//! Raw Gamma API models will be added later and transformed into these.

use serde::Serialize;

/// Fully normalized market representation used by the product.
///
/// This is the ONLY market model exposed to the frontend and future AI systems.
#[derive(Debug, Serialize)]
pub struct AnalyzedMarket {
    /// Stable internal identifier.
    pub id: String,

    // ───────── Core meaning ─────────
    /// Human-readable prediction question.
    pub question: String,

    /// Optional extended description or context.
    pub description: Option<String>,

    /// High-level category (e.g. Crypto, Politics).
    pub category: Option<String>,

    /// Market structure (binary, categorical, scalar).
    pub market_type: MarketType,

    // ───────── Outcomes & pricing ─────────
    /// Possible outcomes for the market.
    pub outcomes: Vec<Outcome>,

    /// Aggregated pricing information.
    pub pricing: PricingInfo,

    // ───────── Market strength ─────────
    /// Total traded volume in USD.
    pub volume_usd: f64,

    /// Estimated available liquidity in USD.
    pub liquidity_usd: f64,

    // ───────── Timing & resolution ─────────
    pub timing: MarketTiming,
    pub resolution: ResolutionInfo,

    // ───────── AI / analysis hooks ─────────
    pub context: MarketContext,
}

/// Supported market types across Polymarket.
#[derive(Debug, Serialize)]
pub enum MarketType {
    /// Two-outcome market (Yes / No).
    Binary,

    /// Multiple discrete outcomes.
    Categorical,

    /// Continuous value with bounds.
    Scalar {
        lower_bound: f64,
        upper_bound: f64,
    },
}

/// A single possible outcome of a market.
#[derive(Debug, Serialize)]
pub struct Outcome {
    /// Display label (e.g. "Yes", "No", "Trump", "Biden").
    pub label: String,

    /// Normalized probability (if available).
    pub probability: Option<f64>,
}

/// Aggregated pricing information for analysis.
#[derive(Debug, Serialize)]
pub struct PricingInfo {
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub mid_price: Option<f64>,
}

/// Timing and lifecycle state of a market.
#[derive(Debug, Serialize)]
pub struct MarketTiming {
    pub end_time: Option<String>,
    pub closed_time: Option<String>,
    pub is_closed: bool,
}

/// Resolution metadata.
#[derive(Debug, Serialize)]
pub struct ResolutionInfo {
    pub resolution_source: Option<String>,
    pub is_resolved: bool,
}

/// Additional contextual metadata useful for AI analysis.
#[derive(Debug, Serialize)]
pub struct MarketContext {
    pub event_title: Option<String>,
    pub tags: Vec<String>,
}
