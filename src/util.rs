use chrono::FixedOffset;

pub fn est() -> FixedOffset {
    chrono::FixedOffset::west(5 * 3600)
}