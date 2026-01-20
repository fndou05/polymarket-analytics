//! Raw -> analyzed market transformation.

use crate::analytics::compute_analytics;
use crate::models::*;

/// Convert a raw Gamma market into the stable API model.
pub fn analyze_market(raw: RawMarket) -> Option<AnalyzedMarket> {
    let base = transform_market(raw)?;
    let analytics = compute_analytics(&base);
    Some(base.with_analytics(analytics))
}

fn transform_market(raw: RawMarket) -> Option<AnalyzedMarketBase> {
    // Some event markets omit marketType; default to binary to avoid dropping valid markets.
    let market_type_key = raw
        .market_type
        .as_deref()
        .unwrap_or("normal")
        .trim()
        .to_ascii_lowercase();
    let market_type = match market_type_key.as_str() {
        "binary" | "normal" => MarketType::Binary,
        "categorical" => MarketType::Categorical,
        "scalar" => MarketType::Scalar {
            lower_bound: raw.lower_bound.unwrap_or(0.0),
            upper_bound: raw.upper_bound.unwrap_or(0.0),
        },
        _ => return None,
    };

    let outcomes = raw
        .outcomes
        .into_iter()
        .enumerate()
        .map(|(i, label)| {
            let price = raw.outcome_prices.get(i).copied().flatten();
            Outcome {
                label,
                price,
                probability: if matches!(market_type, MarketType::Binary) {
                    price
                } else {
                    None
                },
            }
        })
        .collect();

    let status = if raw.closed {
        if raw.resolution_source.is_some() {
            MarketStatus::Resolved
        } else {
            MarketStatus::Closed
        }
    } else {
        MarketStatus::Active
    };

    Some(AnalyzedMarketBase {
        id: raw.id,
        external_slug: raw.slug,
        market_family_key: None,
        interval: None,
        question: raw.question,
        description: raw.description,
        category: normalize_category(raw.category),
        market_type,
        outcomes,
        pricing: PricingInfo {
            best_bid: None,
            best_ask: None,
            mid_price: None,
            last_price: None,
            change_24h: None,
        },
        volume_usd: raw.volume_usd.and_then(|v| v.parse().ok()).unwrap_or(0.0),
        liquidity_usd: raw
            .liquidity_usd
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        timing: MarketTiming {
            end_time: raw.end_time,
            closed_time: None,
            is_closed: raw.closed,
        },
        status,
        resolution: ResolutionInfo {
            resolution_source: raw.resolution_source,
            is_resolved: status == MarketStatus::Resolved,
        },
        context: MarketContext {
            event_title: None,
            tags: vec![],
        },
    })
}

fn normalize_category(category: Option<String>) -> Option<String> {
    let s = category?.trim().to_ascii_lowercase();
    if s.is_empty() {
        None
    } else {
        Some(s[..1].to_uppercase() + &s[1..])
    }
}
