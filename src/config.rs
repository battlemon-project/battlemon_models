use near_sdk::AccountId;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContractConfig {
    top_contract_id: AccountId,
    nft_contract_id: AccountId,
    market_contract_id: AccountId,
}

impl ContractConfig {
    pub fn ids(&self) -> (&AccountId, &AccountId, &AccountId) {
        (
            &self.top_contract_id,
            &self.nft_contract_id,
            &self.market_contract_id,
        )
    }

    pub fn nft_id(&self) -> &AccountId {
        &self.nft_contract_id
    }

    pub fn market_id(&self) -> &AccountId {
        &self.market_contract_id
    }
}
