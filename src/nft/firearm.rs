use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub struct FireArm {
    pub kind: FireArmKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Default,
)]
pub enum FireArmKind {
    #[serde(rename = "FireArms_Assault_Rifle_AA02")]
    FireArmsAssaultRifleAA02,
    #[serde(rename = "FireArms_Assault_Rifle_AA01")]
    FireArmsAssaultRifleAA01,
    #[serde(rename = "FireArms_Handgun_SMG_AA02")]
    FireArmsHandgunSMGAA02,
    #[serde(rename = "FireArms_Grenade_Launcher_AA03")]
    FireArmsGrenadeLauncherAA03,
    #[serde(rename = "FireArms_Revolver_CA01")]
    FireArmsRevolverCA01,
}
