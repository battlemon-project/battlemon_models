use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ColdArm {
    pub token_id: TokenId,
    pub flavour: ColdArmKind,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ColdArmKind {
    #[serde(rename = "ColdArms_Chopper_Knife_KA01")]
    ColdArmsChopperKnifeKa01,
    #[serde(rename = "ColdArms_Grappling_Hook_PA01")]
    ColdArmsGrapplingHookPa01,
    #[serde(rename = "ColdArms_Bottle_Rose_RA01")]
    ColdArmsBottleRoseRa01,
}

impl BuildUrlQuery for ColdArm {}

const COLDARM_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<COLDARM_TRAITS_COUNT> for ColdArm {
    fn from_trait_weights(token_id: &TokenId, [weight]: &[u8; COLDARM_TRAITS_COUNT]) -> Self {
        let flavour = match weight {
            0..=32 => ColdArmKind::ColdArmsChopperKnifeKa01,
            33..=66 => ColdArmKind::ColdArmsGrapplingHookPa01,
            _ => ColdArmKind::ColdArmsBottleRoseRa01,
        };

        Self {
            token_id: token_id.clone(),
            flavour,
        }
    }
}
