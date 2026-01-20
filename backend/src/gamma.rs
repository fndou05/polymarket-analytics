//! Gamma API ingestion via events (stable source for active markets).

use crate::models::{RawEvent, RawMarket};
use reqwest::Client;

const GAMMA_BASE_URL: &str = "https://gamma-api.polymarket.com";
const EVENT_LIMIT: &str = "200"; // small, predictable payload for v1
const USER_AGENT: &str = "polymarket-analytics/1.0";

/// Fetches raw markets by flattening active events from Gamma.
pub async fn fetch_raw_markets_from_events(
    client: &Client,
) -> Result<Vec<RawMarket>, reqwest::Error> {
    let url = format!("{}/events", GAMMA_BASE_URL);
    let events = client
        .get(url)
        .query(&[("active", "true"), ("limit", EVENT_LIMIT)])
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<Vec<RawEvent>>()
        .await?;

    Ok(events.into_iter().flat_map(|event| event.markets).collect())
}
