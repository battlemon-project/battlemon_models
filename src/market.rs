use chrono::Duration;

pub mod ask;
pub mod bid;
pub mod events;
pub mod paid;
pub mod sale;

#[cfg(feature = "market-convert")]
fn convert_to_decimal_near(yocto_near: &near_sdk::json_types::U128) -> rust_decimal::Decimal {
    use rust_decimal::Decimal;
    use rust_decimal::MathematicalOps;

    Decimal::from(yocto_near.0) / Decimal::new(10, 0).powu(24)
}

#[cfg(feature = "market-convert")]
fn convert_from_near_timestamp(timestamp: u64) -> chrono::DateTime<chrono::Utc> {
    use chrono::{DateTime, NaiveDateTime, Utc};
    let duration = Duration::nanoseconds(timestamp);
    let naive = NaiveDateTime::from_timestamp(duration.num_seconds(), 0);
    DateTime::<Utc>::from_utc(naive, Utc)
}
