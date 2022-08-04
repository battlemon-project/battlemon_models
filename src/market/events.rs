use crate::market::sale_contract::SaleForContract;
use crate::market::{ask_contract::Ask, bid_contract::Bid};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "event", content = "data")]
pub enum MarketEventKind {
    Sale(SaleForContract),
    AddBid(Bid),
    AddAsk(Ask),
    RemoveBid(Bid),
    RemoveAsk(Ask),
}
