#[cfg(feature = "nft-contract")]
use crate::nft::ModelKind;
#[cfg(feature = "nft-contract")]
use near_contract_standards::{
    non_fungible_token::metadata::TokenMetadata,
    non_fungible_token::{Token, TokenId},
};
#[cfg(feature = "nft-contract")]
use near_sdk::serde::{Deserialize, Serialize};
#[cfg(feature = "nft-contract")]
use near_sdk::{json_types::Base64VecU8, AccountId};

#[cfg(feature = "nft")]
use sqlx_core::types::Json;
#[cfg(feature = "nft")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftTokenForRest {
    pub token_id: String,
    pub owner_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: String,
    pub media_hash: Option<String>,
    pub copies: Option<String>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub model: Json<ModelKind>,
}

#[cfg(feature = "nft-contract")]
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadataExt {
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
    pub expires_at: Option<String>, // ISO 8601 datetime when token expires
    pub starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
    pub updated_at: Option<String>, // ISO 8601 datetime when token was last updated
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub model: ModelKind,
}

#[cfg(feature = "nft-contract")]
impl TokenMetadataExt {
    pub fn split(self) -> (TokenMetadata, ModelKind) {
        let metadata = TokenMetadata {
            title: self.title,
            description: self.description,
            media: self.media,
            media_hash: self.media_hash,
            copies: self.copies,
            issued_at: self.issued_at,
            expires_at: self.expires_at,
            starts_at: self.starts_at,
            updated_at: self.updated_at,
            extra: self.extra,
            reference: self.reference,
            reference_hash: self.reference_hash,
        };

        (metadata, self.model)
    }
}

#[cfg(feature = "nft-contract")]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenExt {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: Option<TokenMetadata>,
    pub model: ModelKind,
    pub approved_account_ids: Option<std::collections::HashMap<AccountId, u64>>,
}

#[cfg(feature = "nft-contract")]
impl TokenExt {
    pub fn from_parts(token: Token, model: ModelKind) -> TokenExt {
        TokenExt {
            token_id: token.token_id,
            owner_id: token.owner_id,
            metadata: token.metadata,
            approved_account_ids: token.approved_account_ids,
            model,
        }
    }
}

#[cfg(feature = "nft-convert")]
impl TryFrom<TokenExt> for NftTokenForRest {
    type Error = String;

    fn try_from(
        TokenExt {
            token_id,
            owner_id,
            metadata,
            model,
            ..
        }: TokenExt,
    ) -> Result<Self, Self::Error> {
        match metadata {
            Some(TokenMetadata {
                title,
                description,
                media: Some(media),
                media_hash,
                copies,
                issued_at,
                expires_at,
                ..
            }) => Ok(Self {
                token_id,
                owner_id: owner_id.to_string(),
                title,
                description,
                media,
                media_hash: media_hash.map(|hash| near_sdk::serde_json::to_string(&hash).unwrap()),
                copies: copies.map(|copies| copies.to_string()),
                issued_at,
                expires_at,
                model: Json(model),
            }),
            _ => Err("Model is not set".to_string()),
        }
    }
}
