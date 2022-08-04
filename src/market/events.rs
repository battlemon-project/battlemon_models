cfg_if::cfg_if! {
    if #[cfg(feature = "market-events")] {
        use crate::market::sale::SaleForContract;
        use crate::market::ask::AskForContract;
        use crate::market::bid::Bid;

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case", tag = "event", content = "data")]
        pub enum MarketEventKind {
            Sale(SaleForContract),
            AddBid(Bid),
            AddAsk(AskForContract),
            RemoveBid(Bid),
            RemoveAsk(AskForContract),
        }
    }
}
