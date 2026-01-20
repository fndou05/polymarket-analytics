use backend::models::{
    AnalyticsFields, AnalyzedMarket, MarketContext, MarketStatus, MarketTiming, MarketType,
    Outcome, PricingInfo, ResolutionInfo,
};
use backend::pagination::{paginate_markets, StatusFilter};

fn make_market(id: &str, volume: f64, status: MarketStatus) -> AnalyzedMarket {
    AnalyzedMarket {
        id: id.to_string(),
        external_slug: "slug".to_string(),
        market_family_key: None,
        interval: None,
        question: "question".to_string(),
        description: None,
        category: None,
        market_type: MarketType::Binary,
        outcomes: vec![
            Outcome {
                label: "Yes".to_string(),
                price: Some(0.4),
                probability: Some(0.4),
            },
            Outcome {
                label: "No".to_string(),
                price: Some(0.6),
                probability: Some(0.6),
            },
        ],
        pricing: PricingInfo {
            best_bid: None,
            best_ask: None,
            mid_price: None,
            last_price: None,
            change_24h: None,
        },
        volume_usd: volume,
        liquidity_usd: 0.0,
        timing: MarketTiming {
            end_time: None,
            closed_time: None,
            is_closed: false,
        },
        status,
        resolution: ResolutionInfo {
            resolution_source: None,
            is_resolved: false,
        },
        context: MarketContext {
            event_title: None,
            tags: vec![],
        },
        analytics: AnalyticsFields {
            implied_prob_spread: None,
            outcome_entropy: None,
            binary_confidence_score: None,
            is_high_signal: false,
        },
    }
}

#[test]
fn ordering_is_stable_and_deterministic() {
    let markets = vec![
        make_market("b", 10.0, MarketStatus::Active),
        make_market("a", 10.0, MarketStatus::Active),
        make_market("c", 5.0, MarketStatus::Active),
    ];

    let page = paginate_markets(markets, &StatusFilter::All, 10, None);
    let ids: Vec<String> = page.data.into_iter().map(|m| m.id).collect();

    assert_eq!(ids, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn pagination_has_no_overlap_across_pages() {
    let markets = vec![
        make_market("a", 5.0, MarketStatus::Active),
        make_market("b", 4.0, MarketStatus::Active),
        make_market("c", 3.0, MarketStatus::Active),
        make_market("d", 2.0, MarketStatus::Active),
        make_market("e", 1.0, MarketStatus::Active),
    ];

    let page1 = paginate_markets(markets.clone(), &StatusFilter::All, 2, None);
    let cursor = page1.next_cursor.clone().expect("expected cursor");
    let page2 = paginate_markets(markets.clone(), &StatusFilter::All, 2, Some(&cursor));
    let page3 = if let Some(next) = page2.next_cursor.as_deref() {
        paginate_markets(markets.clone(), &StatusFilter::All, 2, Some(next))
    } else {
        paginate_markets(markets.clone(), &StatusFilter::All, 2, None)
    };

    let ids1: Vec<String> = page1.data.iter().map(|m| m.id.clone()).collect();
    let ids2: Vec<String> = page2.data.iter().map(|m| m.id.clone()).collect();
    let ids3: Vec<String> = page3.data.iter().map(|m| m.id.clone()).collect();

    for id in &ids1 {
        assert!(!ids2.contains(id));
    }

    let mut combined = Vec::new();
    combined.extend(ids1);
    combined.extend(ids2);
    combined.extend(ids3);
    combined.sort();
    combined.dedup();

    let full = paginate_markets(markets, &StatusFilter::All, 10, None);
    let mut all_ids: Vec<String> = full.data.into_iter().map(|m| m.id).collect();
    all_ids.sort();
    all_ids.dedup();

    assert_eq!(combined, all_ids);
}

#[test]
fn cursor_resumes_and_has_more_flips() {
    let markets = vec![
        make_market("a", 3.0, MarketStatus::Active),
        make_market("b", 2.0, MarketStatus::Active),
        make_market("c", 1.0, MarketStatus::Active),
    ];

    let page1 = paginate_markets(markets.clone(), &StatusFilter::All, 2, None);
    let cursor = page1.next_cursor.clone().expect("expected cursor");
    let page2 = paginate_markets(markets.clone(), &StatusFilter::All, 2, Some(&cursor));

    assert_eq!(page2.data.len(), 1);
    assert!(page2.has_more == false);

    let end_cursor = page2.data.last().map(|m| m.id.clone()).unwrap();
    let page3 = paginate_markets(markets, &StatusFilter::All, 2, Some(&end_cursor));
    assert!(page3.data.is_empty());
    assert_eq!(page3.has_more, false);
    assert_eq!(page3.next_cursor, None);
}
