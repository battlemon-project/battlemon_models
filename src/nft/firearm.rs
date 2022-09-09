use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FireArm {
    pub token_id: TokenId,
    pub flavour: FireArmKind,
}
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum FireArmKind {
    #[serde(rename = "FireArms_Assault_Rifle_AA01")]
    FireArmsAssaultRifleAA01,
    #[serde(rename = "FireArms_Assault_Rifle_AA02")]
    FireArmsAssaultRifleAA02,
    #[serde(rename = "FireArms_Handgun_SMG_AA04")]
    FireArmsHandgunSMGAA04,
    #[serde(rename = "FireArms_Grenade_Launcher_AA03")]
    FireArmsGrenadeLauncherAA03,
    #[serde(rename = "FireArms_Revolver_CA01")]
    FireArmsRevolverCA01,
    #[serde(rename = "FireArms_Sniper_Rifle_AA05")]
    FireArmsSniperRifleAA05,
}

impl BuildUrlQuery for FireArm {}

const FIREARM_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<FIREARM_TRAITS_COUNT> for FireArm {
    fn from_trait_weights(token_id: &TokenId, [weight]: &[u8; FIREARM_TRAITS_COUNT]) -> Self {
        let flavour = match weight {
            0..=18 => FireArmKind::FireArmsAssaultRifleAA01,
            19..=37 => FireArmKind::FireArmsAssaultRifleAA02,
            38..=56 => FireArmKind::FireArmsHandgunSMGAA04,
            57..=75 => FireArmKind::FireArmsGrenadeLauncherAA03,
            76..=94 => FireArmKind::FireArmsRevolverCA01,
            _ => FireArmKind::FireArmsSniperRifleAA05,
        };

        Self {
            token_id: token_id.clone(),
            flavour,
        }
    }
}
