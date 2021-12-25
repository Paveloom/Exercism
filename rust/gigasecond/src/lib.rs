use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
#[must_use]
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
