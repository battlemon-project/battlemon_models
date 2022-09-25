use crate::nft::token::TokenExt;

#[derive(Debug, Clone, near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(rename_all = "snake_case", tag = "event", content = "data")]
pub enum NftEventKind {
    AssembleNft(TokenExt),
    DisassembleNft(TokenExt),
}