use backend::models::{MarketStatus, MarketType, RawMarket};
use backend::transform::analyze_market;
use serde_json::json;

fn raw_from_json(value: serde_json::Value) -> RawMarket {
    serde_json::from_value(value).expect("failed to parse RawMarket JSON")
}

#[test]
fn binary_market_is_normalized_correctly() {
    let raw = raw_from_json(json!({
        "id": "m1",
        "slug": "binary-test",
        "question": "Binary?",
        "description": null,
        "category": "sports",
        "marketType": "binary",
        "outcomes": "[\"Yes\", \"No\"]",
        "outcomePrices": "[0.4, 0.6]",
        "volumeUSD": "10",
        "liquidityUSD": "5",
        "endDate": null,
        "closed": false,
        "lowerBound": null,
        "upperBound": null,
        "resolutionSource": null
    }));

    let analyzed = analyze_market(raw).expect("expected analyzed market");
    assert!(matches!(analyzed.market_type, MarketType::Binary));
    assert_eq!(analyzed.outcomes.len(), 2);
    assert!(analyzed.outcomes[0].probability.is_some());
    assert_eq!(analyzed.status, MarketStatus::Active);
}

#[test]
fn status_derivation_matches_closed_and_resolution_source() {
    let resolved_raw = raw_from_json(json!({
        "id": "m2",
        "slug": "resolved-test",
        "question": "Resolved?",
        "description": null,
        "category": null,
        "marketType": "binary",
        "outcomes": ["Yes", "No"],
        "outcomePrices": [0.2, 0.8],
        "volumeUSD": "0",
        "liquidityUSD": "0",
        "endDate": null,
        "closed": true,
        "lowerBound": null,
        "upperBound": null,
        "resolutionSource": "source"
    }));
    let resolved = analyze_market(resolved_raw).expect("expected analyzed market");
    assert_eq!(resolved.status, MarketStatus::Resolved);

    let closed_raw = raw_from_json(json!({
        "id": "m3",
        "slug": "closed-test",
        "question": "Closed?",
        "description": null,
        "category": null,
        "marketType": "binary",
        "outcomes": ["Yes", "No"],
        "outcomePrices": [0.2, 0.8],
        "volumeUSD": "0",
        "liquidityUSD": "0",
        "endDate": null,
        "closed": true,
        "lowerBound": null,
        "upperBound": null,
        "resolutionSource": null
    }));
    let closed = analyze_market(closed_raw).expect("expected analyzed market");
    assert_eq!(closed.status, MarketStatus::Closed);
}

#[test]
fn missing_market_type_defaults_to_binary() {
    let mut value = json!({
        "id": "m4",
        "slug": "default-binary",
        "question": "Default?",
        "description": null,
        "category": null,
        "outcomes": ["Yes", "No"],
        "outcomePrices": [0.5, 0.5],
        "volumeUSD": "0",
        "liquidityUSD": "0",
        "endDate": null,
        "closed": false,
        "lowerBound": null,
        "upperBound": null,
        "resolutionSource": null
    });
    value.as_object_mut().unwrap().remove("marketType");

    let raw = raw_from_json(value);
    let analyzed = analyze_market(raw).expect("expected analyzed market");
    assert!(matches!(analyzed.market_type, MarketType::Binary));
}

#[test]
fn empty_strings_do_not_panic() {
    let raw = raw_from_json(json!({
        "id": "m5",
        "slug": "empty-strings",
        "question": "Empty?",
        "description": null,
        "category": "",
        "marketType": "binary",
        "outcomes": ["Yes", "No"],
        "outcomePrices": "[0.4, 0.6]",
        "volumeUSD": "",
        "liquidityUSD": "",
        "endDate": null,
        "closed": false,
        "lowerBound": null,
        "upperBound": null,
        "resolutionSource": null
    }));

    let analyzed = analyze_market(raw).expect("expected analyzed market");
    assert_eq!(analyzed.volume_usd, 0.0);
    assert_eq!(analyzed.liquidity_usd, 0.0);
}
