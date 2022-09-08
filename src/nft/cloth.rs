use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Cloth {
    pub token_id: TokenId,
    pub flavour: ClothKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum ClothKind {
    #[serde(rename = "Cloth_Skull_Belt_PA01")]
    ClothSkullBeltPA01,
    #[serde(rename = "Cloth_Bandolier_MA02")]
    ClothBandolierMA02,
    // #[serde(rename = "Cloth_Poncho_CA01")]
    // ClothPonchoCA01,
    #[serde(rename = "Cloth_Ninja_Waistband_NA01")]
    ClothNinjaWaistbandNA01,
    #[serde(rename = "Cloth_Eastern_Armor_Belt_NA02")]
    ClothEasternArmorBeltNA02,
    #[serde(rename = "Cloth_Cheef_Sash_KA01")]
    ClothCheefSashKA01,
    #[serde(rename = "Cloth_Chain_Gold_RA01")]
    ClothChainGoldRA01,
}

impl BuildUrlQuery for Cloth {}

const CLOTH_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<CLOTH_TRAITS_COUNT> for Cloth {
    fn from_trait_weights(token_id: &TokenId, [weight]: &[u8; CLOTH_TRAITS_COUNT]) -> Self {
        let flavour = match weight {
            0..=16 => ClothKind::ClothSkullBeltPA01,
            17..=32 => ClothKind::ClothBandolierMA02,
            33..=48 => ClothKind::ClothNinjaWaistbandNA01,
            49..=64 => ClothKind::ClothEasternArmorBeltNA02,
            65..=80 => ClothKind::ClothCheefSashKA01,
            _ => ClothKind::ClothChainGoldRA01,
        };

        Self {
            token_id: token_id.clone(),
            flavour,
        }
    }
}
