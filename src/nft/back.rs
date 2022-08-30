use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Back {
    pub kind: BackKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum BackKind {
    #[serde(rename = "Back_Insecticide_bottle_ZA01")]
    BackInsecticideBottleZA01,
    #[serde(rename = "Back_Bomb_Barrel_PA02")]
    BackBombBarrelPA02,
    #[serde(rename = "Back_Tactical_Backpack_MA01")]
    BackTacticalBackpackMA01,
    #[serde(rename = "Back_Edventurer_Backpack_PA01")]
    BackEdventurerBackpackPA01,
}

impl BuildUrlQuery for Back {}

const BACK_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<BACK_TRAITS_COUNT> for Back {
    fn from_trait_weights([weight]: &[u8; BACK_TRAITS_COUNT]) -> Self {
        todo!()
    }
}
