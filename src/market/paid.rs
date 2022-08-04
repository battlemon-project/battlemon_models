cfg_if::cfg_if! {
    if #[cfg(feature = "market")] {
        use crate::market::sale::SaleForDb;
        use rust_decimal::Decimal;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct PaidStatistics {
            #[serde(with = "rust_decimal::serde::str")]
            pub total_trades_volume: Decimal,
            pub total_number_of_trades: usize,
            #[serde(with = "rust_decimal::serde::str")]
            pub top_trade: Decimal,
        }

        #[derive(Serialize, Deserialize)]
        pub struct Paid {
            pub history: Vec<SaleForDb>,
            pub statistics: PaidStatistics,
        }

        impl Paid {
            pub fn new(
                history: Vec<SaleForDb>,
                total_trades_volume: Decimal,
                total_number_of_trades: usize,
                top_trade: Decimal,
            ) -> Self {
                Self {
                    history,
                    statistics: PaidStatistics {
                        total_trades_volume,
                        total_number_of_trades,
                        top_trade,
                    },
                }
            }
        }
    }
}
