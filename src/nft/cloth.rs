use borsh_derive::{borsh_deserialize as BorshDeserialize, borsh_serialize as BorshSerialize};
use serde_derive::{derive_deserialize as Deserialize, derive_serialize as Serialize};

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
pub struct Cloth {
    pub kind: ClothKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub enum ClothKind {
    #[serde(rename = "Cloth_Skull_Belt_PA01")]
    ClothSkullBeltPA01,
    #[serde(rename = "Cloth_Bandolier_MA02")]
    ClothBandolierMA02,
    #[serde(rename = "Cloth_Poncho_CA01")]
    ClothPonchoCA01,
    #[serde(rename = "Cloth_Ninja_Waistband_NA01")]
    ClothNinjaWaistbandNA01,
    #[serde(rename = "Cloth_Eastern_Armor_Belt_NA02")]
    ClothEasternArmorBeltNA02,
    #[serde(rename = "Cloth_Cheef_Sash_KA01")]
    ClothCheefSashKA01,
    #[serde(rename = "Cloth_Chain_Gold_RA01")]
    ClothChainGoldRA01,
}
