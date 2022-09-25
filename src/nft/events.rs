#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "snake_case")]
pub enum NftEventKind {
    NftMint,
    NftBurn,
    AssembleNft,
    DisassembleNft,
}

#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum StandardKind {
    #[serde(rename = "nep171")]
    Nep171,
}

#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum VersionKind {
    #[serde(rename = "1.0.0")]
    V1_0_0,
}

#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum NftEventLogKind {
    NftMintLog {
        owner_id: String,
        token_ids: Vec<String>,
        memo: Option<String>,
    },
}

#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftEvent {
    pub standard: StandardKind,
    pub version: VersionKind,
    pub event: NftEventKind,
    pub data: Option<Vec<NftEventLogKind>>,
}
