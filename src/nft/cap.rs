use crate::nft::{BuildUrlQuery, FromTraitWeights};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Cap {
    pub token_id: TokenId,
    pub flavour: CapKind,
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
    fn from_trait_weights(token_id: &TokenId, [weight]: &[u8; CAP_TRAITS_COUNT]) -> Self {
        let flavour = match weight {
            0..=5 => CapKind::CapBaseballCapRedRA01,
            6..=11 => CapKind::CapLadleZA01,
            12..=17 => CapKind::CapCheefHatKA01,
            18..=23 => CapKind::CapConeArmoredHatNA03,
            24..=29 => CapKind::CapCowboyHatCA01,
            30..=35 => CapKind::CapSheriffHatCA02,
            36..=41 => CapKind::CapMilitaryCapMA05,
            42..=47 => CapKind::CapSpecialForcesBeretMA02,
            48..=53 => CapKind::CapTankHelmetMA03,
            54..=59 => CapKind::CapMilitaryHelmetMA04,
            60..=65 => CapKind::CapMetallicConeHatNA04,
            66..=71 => CapKind::CapAssaultHelmetMA01,
            72..=77 => CapKind::CapCaneConeHatNA02,
            78..=83 => CapKind::CapCockedHatPA01,
            _ => CapKind::CapPirateBandanaPA02,
        };

        Self {
            token_id: token_id.clone(),
            flavour,
        }
    }
}
