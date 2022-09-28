use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Set {
    pub token_id: TokenId,
    pub flavour: SetKind,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum SetKind {
    Set1,
    Set2,
}

impl BuildUrlQuery for Set {}

const COLDARM_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<COLDARM_TRAITS_COUNT> for Set {
    fn from_trait_weights(token_id: &TokenId, [weight]: &[u8; COLDARM_TRAITS_COUNT]) -> Self {
        let flavour = match weight {
            0..=50 => SetKind::Set1,
            _ => SetKind::Set2,
        };

        Self {
            token_id: token_id.clone(),
            flavour,
        }
    }
}
