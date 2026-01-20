use crate::models::{AnalyzedMarket, MarketStatus, PaginatedResponse};

#[derive(Debug, Clone, Copy)]
pub enum StatusFilter {
    Active,
    Closed,
    Resolved,
    Inactive,
    All,
}

impl StatusFilter {
    pub fn matches(&self, status: MarketStatus) -> bool {
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

pub fn parse_status_filter(value: Option<String>) -> StatusFilter {
    match value.as_deref().map(|v| v.trim().to_ascii_lowercase()) {
        Some(ref v) if v == "closed" => StatusFilter::Closed,
        Some(ref v) if v == "resolved" => StatusFilter::Resolved,
        Some(ref v) if v == "inactive" => StatusFilter::Inactive,
        Some(ref v) if v == "all" => StatusFilter::All,
        _ => StatusFilter::Active,
    }
}

pub fn paginate_markets(
    markets: Vec<AnalyzedMarket>,
    filter: &StatusFilter,
    limit: usize,
    cursor: Option<&str>,
) -> PaginatedResponse<AnalyzedMarket> {
    let mut filtered = apply_filter(filter, markets);
    // Stable ordering: volume DESC, then id ASC.
    filtered.sort_by(|a, b| {
        b.volume_usd
            .partial_cmp(&a.volume_usd)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.id.cmp(&b.id))
    });

    // Cursor points to the last seen id; start after it.
    let start_index = if let Some(cursor) = cursor {
        filtered
            .iter()
            .position(|m| m.id == cursor)
            .map(|i| i + 1)
            .unwrap_or(0)
    } else {
        0
    };

    let page = filtered
        .iter()
        .skip(start_index)
        .take(limit)
        .cloned()
        .collect::<Vec<_>>();

    let has_more = start_index + limit < filtered.len();
    let next_cursor = if has_more {
        page.last().map(|m| m.id.clone())
    } else {
        None
    };

    PaginatedResponse {
        data: page,
        next_cursor,
        has_more,
    }
}

fn apply_filter(filter: &StatusFilter, markets: Vec<AnalyzedMarket>) -> Vec<AnalyzedMarket> {
    match filter {
        StatusFilter::All => markets,
        _ => markets
            .into_iter()
            .filter(|m| filter.matches(m.status))
            .collect(),
    }
}
