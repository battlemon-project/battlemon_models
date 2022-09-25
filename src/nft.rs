#[cfg(feature = "nft-contract")]
use near_contract_standards::non_fungible_token::TokenId;
#[cfg(feature = "nft-contract")]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[cfg(feature = "nft-contract")]
use near_sdk::serde::{Deserialize, Serialize};

#[cfg(feature = "nft-contract")]
pub use back::*;
#[cfg(feature = "nft-contract")]
pub use cap::*;
#[cfg(feature = "nft-contract")]
pub use cloth::*;
#[cfg(feature = "nft-contract")]
pub use coldarm::*;
#[cfg(feature = "nft-events")]
pub use events::*;
#[cfg(feature = "nft-contract")]
pub use firearm::*;
#[cfg(feature = "nft-contract")]
pub use lemon::*;
pub use token::*;

#[cfg(feature = "nft-contract")]
mod back;
#[cfg(feature = "nft-contract")]
mod cap;
#[cfg(feature = "nft-contract")]
mod cloth;
#[cfg(feature = "nft-contract")]
mod coldarm;
#[cfg(feature = "nft-events")]
mod events;
#[cfg(feature = "nft-contract")]
mod firearm;
#[cfg(feature = "nft-contract")]
mod lemon;
mod token;

#[cfg(feature = "nft-contract")]
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

#[cfg(feature = "nft-contract")]
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

#[cfg(feature = "nft-contract")]
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

#[cfg(feature = "nft-contract")]
pub trait BuildUrlQuery {
    fn build_url_query(&self) -> String
    where
        Self: Serialize,
    {
        let value =
            near_sdk::serde_json::to_value(self).expect("Couldn't serialize `Self` into `Value`");
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

#[cfg(feature = "nft-contract")]
pub trait FromTraitWeights<const COUNT: usize> {
    fn from_trait_weights(token_id: &TokenId, weights: &[u8; COUNT]) -> Self;
}
