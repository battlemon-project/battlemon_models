use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Ask {
    pub id: String,
    pub account_id: AccountId,
    pub token_id: TokenId,
    pub approval_id: u64,
    pub price: U128,
}

impl Ask {
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: U128) -> Self {
        Self {
            id: crate::helpers_contract::generate_id(),
            account_id: owner_id,
            token_id,
            approval_id,
            price,
        }
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn approval_id(&self) -> Option<u64> {
        Some(self.approval_id)
    }

    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub fn price(&self) -> u128 {
        self.price.0
    }
}
