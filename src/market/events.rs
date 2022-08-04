#[cfg(feature = "market-events")]
use crate::market::ask::AskForContract;
#[cfg(feature = "market-events")]
use crate::market::bid::BidForContract;
#[cfg(feature = "market-events")]
use crate::market::sale::SaleForContract;

#[cfg(feature = "market-events")]
#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "snake_case", tag = "event", content = "data")]
pub enum MarketEventKind {
    Sale(SaleForContract),
    AddBid(BidForContract),
    AddAsk(AskForContract),
    RemoveBid(BidForContract),
    RemoveAsk(AskForContract),
}
