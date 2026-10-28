//! Merged watched playback intervals per lesson.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatchedRange {
    pub start_ms: i64,
    pub end_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackRangeError {
    InvalidRange,
}

/// Insert `new_range` into `ranges` and return a sorted, merged list.
pub fn union_watched_ranges(
    ranges: &[WatchedRange],
    new_range: WatchedRange,
) -> Result<Vec<WatchedRange>, PlaybackRangeError> {
    validate_range(&new_range)?;
    let mut merged = ranges.to_vec();
    merged.push(new_range);
    merged.sort_by_key(|r| r.start_ms);
    Ok(coalesce_sorted(merged))
}

pub fn watched_ms(ranges: &[WatchedRange]) -> i64 {
    ranges.iter().map(|r| r.end_ms - r.start_ms).sum()
}

/// Rounded percent when duration is known; `None` when duration is missing or zero.
pub fn watched_percent(watched_ms: i64, duration_ms: Option<i64>) -> Option<i64> {
    let duration = duration_ms?;
    if duration <= 0 {
        return None;
    }
    let percent = ((watched_ms as f64 / duration as f64) * 100.0).round() as i64;
    Some(percent.min(100))
}

fn validate_range(range: &WatchedRange) -> Result<(), PlaybackRangeError> {
    if range.start_ms < 0 || range.end_ms <= range.start_ms {
        return Err(PlaybackRangeError::InvalidRange);
    }
    Ok(())
}

fn coalesce_sorted(ranges: Vec<WatchedRange>) -> Vec<WatchedRange> {
    if ranges.is_empty() {
        return ranges;
    }
    let mut out = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];
    for next in ranges.into_iter().skip(1) {
        if next.start_ms <= current.end_ms {
            current.end_ms = current.end_ms.max(next.end_ms);
        } else {
            out.push(current);
            current = next;
        }
    }
    out.push(current);
    out
}
