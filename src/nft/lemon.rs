use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;

use crate::nft::BuildUrlQuery;
use crate::nft::{Back, Cap, ClothKind, ColdArm, FireArm};

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Lemon {
    pub exo: Exo,
    pub eyes: Eyes,
    pub face: Face,
    pub teeth: Teeth,
    pub firearm: Option<FireArm>,
    pub coldarm: Option<ColdArm>,
    pub cloth: Option<ClothKind>,
    pub cap: Option<Cap>,
    pub back: Option<Back>,
}

impl Lemon {
    pub const TRAITS_COUNT: usize = 4;

    pub fn from_random(nums: &[u8; Self::TRAITS_COUNT]) -> Self {
        let [exo, eyes, face, teeth] = nums;

        let exo = match exo {
            0..=33 => Exo::BA01,
            34..=66 => Exo::MA01,
            _ => Exo::ZA01,
        };

        let eyes = match eyes {
            0..=33 => Eyes::A01,
            34..=66 => Eyes::B01,
            _ => Eyes::Z01,
        };

        let face = match face {
            0..=33 => Face::A01,
            34..=66 => Face::B01,
            _ => Face::Z01,
        };

        let teeth = match teeth {
            0..=33 => Teeth::A01,
            34..=66 => Teeth::B01,
            _ => Teeth::Z01,
        };

        Self {
            exo,
            eyes,
            face,
            teeth,
            ..Default::default()
        }
    }
}

impl BuildUrlQuery for Lemon {
    fn build_url_query(&self) -> String {
        let value = serde_json::to_value(self).expect("Couldn't serialize `Lemon` into `Value`");
        let exo = value
            .get("exo")
            .expect("Couldn't get exo from value")
            .as_str()
            .expect("Couldn't convert to str");
        let cap = value
            .get("cap")
            .expect("Couldn't get cap from value")
            .as_str()
            .expect("Couldn't convert to str");
        let cloth = value
            .get("cloth")
            .expect("Couldn't get cloth from value")
            .as_str()
            .expect("Couldn't convert to str");
        let eyes = value
            .get("eyes")
            .expect("Couldn't get eyes from value")
            .as_str()
            .expect("Couldn't convert to str");
        let head = value
            .get("head")
            .expect("Couldn't get head from value")
            .as_str()
            .expect("Couldn't convert to str");
        let teeth = value
            .get("teeth")
            .expect("Couldn't get teeth from value")
            .as_str()
            .expect("Couldn't convert to str");

        format!("?background=red&exo={exo}&cap={cap}&cloth={cloth}&eyes={eyes}&head={head}&teeth={teeth}")
    }
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Exo {
    #[serde(rename = "Exo_Snowwhite_Exoskeleton_AA02")]
    ExoSnowwhiteExoSkeletonAA02,
    #[serde(rename = "Exo_Steel_Exoskeleton_AA01")]
    ExoSteelExoskeletonAA01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Eyes {
    #[serde(rename = "Eyes_Blue_AA01")]
    EyesBlueAA01,
    #[serde(rename = "Eyes_Green_AA02")]
    EyesGreenAA02,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Face {
    #[serde(rename = "ARM1_Head_A01")]
    A01,
    #[serde(rename = "ARM1_Head_B01")]
    B01,
    #[serde(rename = "ARM1_Head_Z01")]
    Z01,
}

#[derive(
    Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Debug, Default,
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
