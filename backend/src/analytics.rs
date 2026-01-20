use crate::models::{AnalyticsFields, AnalyzedMarketBase, MarketType};

pub fn compute_analytics(market: &AnalyzedMarketBase) -> AnalyticsFields {
    let implied_prob_spread = compute_prob_spread(market);
    let outcome_entropy = compute_entropy(market);
    let binary_confidence_score = compute_binary_confidence(market);

    let is_high_signal = binary_confidence_score.map(|c| c >= 0.8).unwrap_or(false)
        && market.liquidity_usd >= 10_000.0;

    AnalyticsFields {
        implied_prob_spread,
        outcome_entropy,
        binary_confidence_score,
        is_high_signal,
    }
}

fn compute_prob_spread(market: &AnalyzedMarketBase) -> Option<f64> {
    // Binary only: absolute difference between the two outcome probabilities.
    if !matches!(market.market_type, MarketType::Binary) {
        return None;
    }

    let probs: Vec<f64> = market
        .outcomes
        .iter()
        .filter_map(|o| o.probability)
        .collect();

    if probs.len() != 2 {
        return None;
    }

    Some((probs[0] - probs[1]).abs())
}

fn compute_entropy(market: &AnalyzedMarketBase) -> Option<f64> {
    // Shannon entropy over available outcome probabilities.
    let probs: Vec<f64> = market
        .outcomes
        .iter()
        .filter_map(|o| o.probability)
        .filter(|p| *p > 0.0)
        .collect();

    if probs.len() < 2 {
        return None;
    }

    let entropy = -probs.into_iter().map(|p| p * p.ln()).sum::<f64>();

    Some(entropy)
}

fn compute_binary_confidence(market: &AnalyzedMarketBase) -> Option<f64> {
    // Binary only: max probability as a simple confidence score.
    if !matches!(market.market_type, MarketType::Binary) {
        return None;
    }

    let probs: Vec<f64> = market
        .outcomes
        .iter()
        .filter_map(|o| o.probability)
        .collect();

    if probs.len() != 2 {
        return None;
    }

    probs.into_iter().reduce(f64::max)
}
