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
