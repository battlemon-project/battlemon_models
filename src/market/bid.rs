cfg_if::cfg_if! {
    if #[cfg(feature = "market-contract")] {
        use near_contract_standards::non_fungible_token::TokenId;
        use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
        use near_sdk::json_types::U128;
        use near_sdk::serde::{Deserialize, Serialize};
        use near_sdk::{env, AccountId};

        #[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
        #[serde(crate = "near_sdk::serde")]
        pub struct Bid {
            pub id: String,
            pub token_id: TokenId,
            pub expire_at: Option<u64>,
            pub account_id: AccountId,
            pub price: U128,
            pub create_at: u64,
        }

        impl Bid {
            pub fn new(token_id: TokenId, expire_at: Option<u64>) -> Self {
                Self {
                    id: crate::helpers_contract::generate_id(),
                    token_id,
                    expire_at,
                    account_id: env::predecessor_account_id(),
                    price: U128(env::attached_deposit()),
                    create_at: env::block_timestamp(),
                }
            }

            pub fn account_id(&self) -> &AccountId {
                &self.account_id
            }

            pub fn price(&self) -> u128 {
                self.price.0
            }

            pub fn token_id(&self) -> &TokenId {
                &self.token_id
            }

            pub fn create_at(&self) -> u64 {
                self.create_at
            }
        }
    }
}
