use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub struct ColdArm {
    pub kind: ColdArmKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug, Default,
)]
pub enum ColdArmKind {
    #[serde(rename = "ColdArms_Chopper_Knife_KA01")]
    ColdArmsChopperKnifeKa01,
    #[serde(rename = "ColdArms_Grappling_Hook_PA01")]
    ColdArmsGrapplingHookPa01,
    #[serde(rename = "ColdArms_Bottle_Rose_RA01")]
    ColdArmsBottleRoseRa01,
}
