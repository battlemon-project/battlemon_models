use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Cap {
    pub kind: CapKind,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum CapKind {
    #[serde(rename = "Cap_Baseball_Cap_Red_RA01")]
    CapBaseballCapRedRA01,
    #[serde(rename = "Cap_Ladle_ZA01")]
    CapLadleZA01,
    #[serde(rename = "Cap_Cheef_Hat_KA01")]
    CapCheefHatKA01,
    #[serde(rename = "Cap_Space_Helmet_SA01")]
    CapSpaceHelmetSA01,
    #[serde(rename = "Cap_Cone_Armored_Hat_NA03")]
    CapConeArmoredHatNA03,
    #[serde(rename = "Cap_Cowboy_Hat_CA01")]
    CapCowboyHatCA01,
    #[serde(rename = "Cap_Sheriff_Hat_CA02")]
    CapSheriffHatCA02,
    #[serde(rename = "Cap_Military_Cap_MA05")]
    CapMilitaryCapMA05,
    #[serde(rename = "Cap_Special_Forces_Beret_MA02")]
    CapSpecialForcesBeretMA02,
    #[serde(rename = "Cap_Tank_Helmet_MA03")]
    CapTankHelmetMA03,
    #[serde(rename = "Cap_Military_Helmet_MA04")]
    CapMilitaryHelmetMA04,
    #[serde(rename = "Cap_Metallic_Cone_Hat_NA04")]
    CapMetallicConeHatNA04,
    #[serde(rename = "Cap_Assault_Helmet_MA01")]
    CapAssaultHelmetMA01,
    #[serde(rename = "Cap_Cane_Cone_Hat_NA02")]
    CapCaneConeHatNA02,
    #[serde(rename = "Cap_Cocked_Hat_PA01")]
    CapCockedHatPA01,
    #[serde(rename = "Cap_Pirate_Bandana_PA02")]
    CapPirateBandanaPA02,
}

impl BuildUrlQuery for Cap {}

const CAP_TRAITS_COUNT: usize = 1;
impl FromTraitWeights<CAP_TRAITS_COUNT> for Cap {
    fn from_trait_weights([weight]: &[u8; CAP_TRAITS_COUNT]) -> Self {
        todo!()
    }
}
