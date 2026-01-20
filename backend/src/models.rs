//! Domain models and Gamma API models.

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyticsFields {
    pub implied_prob_spread: Option<f64>,
    pub outcome_entropy: Option<f64>,
    pub binary_confidence_score: Option<f64>,
    pub is_high_signal: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyzedMarketBase {
    pub id: String,
    pub external_slug: String,
    pub market_family_key: Option<String>,
    pub interval: Option<MarketInterval>,
    pub question: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub market_type: MarketType,
    pub outcomes: Vec<Outcome>,
    pub pricing: PricingInfo,
    pub volume_usd: f64,
    pub liquidity_usd: f64,
    pub timing: MarketTiming,
    pub status: MarketStatus,
    pub resolution: ResolutionInfo,
    pub context: MarketContext,
}

impl AnalyzedMarketBase {
    pub fn with_analytics(self, analytics: AnalyticsFields) -> AnalyzedMarket {
        AnalyzedMarket {
            id: self.id,
            external_slug: self.external_slug,
            market_family_key: self.market_family_key,
            interval: self.interval,
            question: self.question,
            description: self.description,
            category: self.category,
            market_type: self.market_type,
            outcomes: self.outcomes,
            pricing: self.pricing,
            volume_usd: self.volume_usd,
            liquidity_usd: self.liquidity_usd,
            timing: self.timing,
            status: self.status,
            resolution: self.resolution,
            context: self.context,
            analytics,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalyzedMarket {
    pub id: String,
    pub external_slug: String,
    pub market_family_key: Option<String>,
    pub interval: Option<MarketInterval>,
    pub question: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub market_type: MarketType,
    pub outcomes: Vec<Outcome>,
    pub pricing: PricingInfo,
    pub volume_usd: f64,
    pub liquidity_usd: f64,
    pub timing: MarketTiming,
    pub status: MarketStatus,
    pub resolution: ResolutionInfo,
    pub context: MarketContext,
    pub analytics: AnalyticsFields,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarketType {
    Binary,
    Categorical,
    Scalar { lower_bound: f64, upper_bound: f64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarketInterval {
    M15,
    H1,
    H4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outcome {
    pub label: String,
    pub price: Option<f64>,
    pub probability: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct OutcomeSnapshot {
    pub label: String,
    pub price: Option<f64>,
    pub probability: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PricingInfo {
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub mid_price: Option<f64>,
    pub last_price: Option<f64>,
    pub change_24h: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketTiming {
    pub end_time: Option<String>,
    pub closed_time: Option<String>,
    pub is_closed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Active,
    Closed,
    Resolved,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolutionInfo {
    pub resolution_source: Option<String>,
    pub is_resolved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketContext {
    pub event_title: Option<String>,
    pub tags: Vec<String>,
}

/// Time-series snapshot of a market at a specific instant.
/// Cadence (design only): active every 5 min, high-signal every 1 min, closed/resolved stops.
/// Rules: append-only, immutable, observational, Gamma is source of truth, backfills allowed.
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct MarketSnapshot {
    pub market_id: String,
    pub captured_at: String,
    pub status: MarketStatus,
    pub outcomes: Vec<OutcomeSnapshot>,
    pub volume_usd: f64,
    pub liquidity_usd: f64,
    pub analytics: AnalyticsFields,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct RawMarket {
    pub id: String,
    pub slug: String,
    pub question: String,
    pub description: Option<String>,
    pub category: Option<String>,

    #[serde(rename = "marketType", default)]
    pub market_type: Option<String>,

    #[serde(deserialize_with = "deserialize_outcomes")]
    pub outcomes: Vec<String>,

    #[serde(
        rename = "outcomePrices",
        default,
        deserialize_with = "deserialize_outcome_prices"
    )]
    pub outcome_prices: Vec<Option<f64>>,

    #[serde(rename = "volumeUSD", default)]
    pub volume_usd: Option<String>,

    #[serde(rename = "liquidityUSD", default)]
    pub liquidity_usd: Option<String>,

    #[serde(rename = "endDate")]
    pub end_time: Option<String>,

    #[serde(rename = "closed")]
    pub closed: bool,

    #[serde(
        rename = "lowerBound",
        default,
        deserialize_with = "deserialize_opt_f64"
    )]
    pub lower_bound: Option<f64>,

    #[serde(
        rename = "upperBound",
        default,
        deserialize_with = "deserialize_opt_f64"
    )]
    pub upper_bound: Option<f64>,

    #[serde(rename = "resolutionSource")]
    pub resolution_source: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RawEvent {
    pub id: String,
    pub title: String,
    pub markets: Vec<RawMarket>,
}

fn deserialize_outcomes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .map(|v| {
                v.as_str()
                    .map(String::from)
                    .ok_or_else(|| de::Error::custom("invalid outcome"))
            })
            .collect(),
        serde_json::Value::String(s) => serde_json::from_str(&s).map_err(de::Error::custom),
        _ => Err(de::Error::custom("invalid outcomes format")),
    }
}

fn deserialize_outcome_prices<'de, D>(deserializer: D) -> Result<Vec<Option<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    fn parse_val<E>(v: serde_json::Value) -> Result<Option<f64>, E>
    where
        E: de::Error,
    {
        match v {
            serde_json::Value::Null => Ok(None),
            serde_json::Value::Number(n) => Ok(n.as_f64()),
            serde_json::Value::String(s) => {
                if s.trim().is_empty() {
                    Ok(None)
                } else {
                    s.parse::<f64>().map(Some).map_err(E::custom)
                }
            }
            _ => Err(E::custom("invalid outcome price")),
        }
    }

    match value {
        None => Ok(Vec::new()),
        Some(serde_json::Value::Array(arr)) => arr.into_iter().map(parse_val::<D::Error>).collect(),
        Some(serde_json::Value::String(s)) => {
            if s.trim().is_empty() {
                Ok(Vec::new())
            } else {
                let parsed: serde_json::Value =
                    serde_json::from_str(&s).map_err(de::Error::custom)?;
                match parsed {
                    serde_json::Value::Array(arr) => {
                        arr.into_iter().map(parse_val::<D::Error>).collect()
                    }
                    _ => Err(de::Error::custom("invalid outcome prices format")),
                }
            }
        }
        _ => Err(de::Error::custom("invalid outcome prices format")),
    }
}

fn deserialize_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        Some(serde_json::Value::Number(n)) => Ok(n.as_f64()),
        Some(serde_json::Value::String(s)) => Ok(s.parse::<f64>().ok()),
        _ => Ok(None),
    }
}
