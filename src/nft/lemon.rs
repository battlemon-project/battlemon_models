use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::nft::{Back, Cap, Cloth, ColdArm, FireArm};
use crate::nft::{BuildUrlQuery, FromTraitWeights};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Lemon {
    pub exo: Exo,
    pub eyes: Eyes,
    pub head: Head,
    pub face: Face,
    pub teeth: Teeth,
    pub fire_arm: Option<FireArm>,
    pub cold_arm: Option<ColdArm>,
    pub cloth: Option<Cloth>,
    pub cap: Option<Cap>,
    pub back: Option<Back>,
}

const LEMON_TRAITS_COUNT: usize = 5;
impl FromTraitWeights<LEMON_TRAITS_COUNT> for Lemon {
    fn from_trait_weights(
        _: &TokenId,
        [exo, eyes, head, face, teeth]: &[u8; LEMON_TRAITS_COUNT],
    ) -> Self {
        let exo = match exo {
            0..=49 => Exo::ExoSnowwhiteExoSkeletonAA02,
            _ => Exo::ExoSteelExoskeletonAA01,
        };

        let eyes = match eyes {
            0..=49 => Eyes::EyesBlueAA01,
            _ => Eyes::EyesGreenAA02,
        };

        let head = match head {
            0..=49 => Head::HeadFreshLemonAA01,
            _ => Head::HeadZombieZA01,
        };

        let face = match face {
            0..=24 => Face::FaceSunglassesRA01,
            25..=50 => Face::FaceGasMaskMA01,
            51..=75 => Face::FaceNinjaBalaclavaNA01,
            _ => Face::FaceCowboyScarfCA01,
        };

        let teeth = match teeth {
            0..=24 => Teeth::TeethGrgaAA02,
            25..=50 => Teeth::TeethHollywoodAA01,
            51..=75 => Teeth::TeethOldstyleAA04,
            _ => Teeth::TeethSharpAA03,
        };

        Self {
            exo,
            eyes,
            head,
            face,
            teeth,
            fire_arm: None,
            cold_arm: None,
            cloth: None,
            cap: None,
            back: None,
        }
    }
}

impl BuildUrlQuery for Lemon {}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Exo {
    #[serde(rename = "Exo_Snowwhite_Exoskeleton_AA02")]
    ExoSnowwhiteExoSkeletonAA02,
    #[serde(rename = "Exo_Steel_Exoskeleton_AA01")]
    ExoSteelExoskeletonAA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Head {
    #[serde(rename = "Head_Fresh_Lemon_AA01")]
    HeadFreshLemonAA01,
    #[serde(rename = "Head_Zombie_ZA01")]
    HeadZombieZA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Eyes {
    #[serde(rename = "Eyes_Blue_AA01")]
    EyesBlueAA01,
    #[serde(rename = "Eyes_Green_AA02")]
    EyesGreenAA02,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Face {
    #[serde(rename = "Face_Ninja_Balaclava_NA_01")]
    FaceNinjaBalaclavaNA01,
    #[serde(rename = "Face_Gas_Mask_MA01")]
    FaceGasMaskMA01,
    #[serde(rename = "Face_Cowboy_Scarf_CA01")]
    FaceCowboyScarfCA01,
    #[serde(rename = "Face_Sunglasses_RA01")]
    FaceSunglassesRA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Teeth {
    #[serde(rename = "Teeth_Grga_AA02")]
    TeethGrgaAA02,
    #[serde(rename = "Teeth_Hollywood_AA01")]
    TeethHollywoodAA01,
    #[serde(rename = "Teeth_Oldstyle_AA04")]
    TeethOldstyleAA04,
    #[serde(rename = "Teeth_Sharp_AA03")]
    TeethSharpAA03,
}
