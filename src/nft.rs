use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

pub use back::*;
pub use cap::*;
pub use cloth::*;
pub use coldarm::*;
pub use firearm::*;
pub use lemon::*;

mod back;
mod cap;
mod cloth;
mod coldarm;
mod firearm;
mod lemon;

#[derive(Serialize, Deserialize, Clone, Copy, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum NftKind {
    Lemon,
    FireArm,
    ColdArm,
    Cloth,
    Back,
    Cap,
}

#[derive(Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case", tag = "kind")]
pub enum ModelKind {
    Lemon(Lemon),
    FireArm(FireArm),
    ColdArm(ColdArm),
    Cloth(Cloth),
    Back(Back),
    Cap(Cap),
}

impl BuildUrlQuery for ModelKind {
    fn build_url_query(&self) -> String {
        match self {
            Self::Lemon(lemon) => lemon.build_url_query(),
            Self::FireArm(firearm) => firearm.build_url_query(),
            Self::ColdArm(coldarm) => coldarm.build_url_query(),
            Self::Cloth(cloth) => cloth.build_url_query(),
            Self::Back(back) => back.build_url_query(),
            Self::Cap(cap) => cap.build_url_query(),
        }
    }
}

pub trait BuildUrlQuery {
    fn build_url_query(&self) -> String
    where
        Self: Serialize,
    {
        let value = serde_json::to_value(self).expect("Couldn't serialize `Self` into `Value`");
        let map = value
            .as_object()
            .expect("Failed to convert `Value` into `Map`");

        let mut query = String::from("?");
        for (idx, (key, value)) in map.iter().enumerate() {
            if idx != 0 {
                query.push('&');
            }
            query.push_str(&format!("{}={}", key, value));
        }

        query
    }
}

pub trait FromTraitWeights<const COUNT: usize> {
    fn from_trait_weights(token_id: &TokenId, weights: &[u8; COUNT]) -> Self;
}
