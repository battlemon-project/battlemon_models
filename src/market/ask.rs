#[mixin::declare]
#[derive(Debug, Clone, PartialEq)]
struct AskAbstract {
    pub id: String,
}

#[cfg(feature = "market")]
use rust_decimal::Decimal;
#[cfg(feature = "market")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "market")]
#[mixin::declare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct AskAbstractForRestAndDb {
    pub id: String,
    pub token_id: String,
    pub account_id: String,
    pub approval_id: i64,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "market")]
#[mixin::insert(AskAbstractForRestAndDb)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AskForRest {}

#[cfg(feature = "market")]
#[mixin::insert(AskAbstractForRestAndDb)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AskForDb {}

#[cfg(feature = "faker")]
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
#[cfg(feature = "faker")]
use fake::{locales::EN, Dummy, Fake, Faker};
#[cfg(feature = "faker")]
use rand::Rng;
//
// impl Dummy<Faker> for AskForRest {
//     fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
//         let scale = rng.gen_range(0..=24);
//         let lo = rng.gen();
//         let mid = rng.gen();
//         let price = Decimal::from_parts(lo, mid, 0, false, scale);
//         Self {
//             id:
//             prev_owner: format!("{}.near", Word().fake::<String>()),
//             curr_owner: format!("{}.near", Word().fake::<String>()),
//             token_id: NumberWithFormat(EN, "^########").fake::<String>(),
//             price,
//         }
//     }
// }
//
// impl Dummy<Faker> for AskForDb {
//     fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
//         let sale_without_id: SaleForInserting = Faker.fake();
//         Self {
//             id: Faker.fake(),
//             date: DateTime().fake(),
//             prev_owner: sale_without_id.prev_owner,
//             curr_owner: sale_without_id.curr_owner,
//             token_id: sale_without_id.token_id,
//             price: sale_without_id.price,
//         }
//     }
// }

#[cfg(feature = "market-contract")]
use near_contract_standards::non_fungible_token::TokenId;
#[cfg(feature = "market-contract")]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[cfg(feature = "market-contract")]
use near_sdk::json_types::U128;
#[cfg(feature = "market-contract")]
use near_sdk::AccountId;

#[cfg(feature = "market-contract")]
#[mixin::insert(AskAbstract)]
#[derive(
    BorshSerialize,
    BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
#[serde(crate = "near_sdk::serde")]
pub struct AskForContract {
    pub token_id: TokenId,
    pub account_id: AccountId,
    pub approval_id: u64,
    pub price: U128,
}

#[cfg(feature = "market-contract")]
impl AskForContract {
    pub fn new(owner_id: AccountId, token_id: TokenId, approval_id: u64, price: U128) -> Self {
        Self {
            id: crate::helpers_contract::generate_id(),
            account_id: owner_id,
            token_id,
            approval_id,
            price,
        }
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn approval_id(&self) -> Option<u64> {
        Some(self.approval_id)
    }

    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub fn price(&self) -> u128 {
        self.price.0
    }
}

#[cfg(feature = "market-convert")]
impl From<AskForContract> for crate::market::ask::AskForRest {
    fn from(ask: AskForContract) -> Self {
        let price = crate::market::convert_to_decimal_near(&ask.price);

        Self {
            id: ask.id,
            token_id: ask.token_id.to_string(),
            account_id: ask.account_id.to_string(),
            approval_id: ask.approval_id as i64,
            price,
        }
    }
}

#[cfg(all(test, feature = "market-convert"))]
mod tests {
    use super::*;

    #[test]
    fn test_convert_from_ask_for_contract_to_sale_for_inserting() {
        let one_near = 1_000_000_000_000_000_000_000_000;
        let prev_owner = "alice.near".to_string();
        let curr_owner = "bob.near".to_string();
        let token_id = "123456789".to_string();
        let sale_for_contract = AskForContract {
            prev_owner: prev_owner.clone(),
            curr_owner: curr_owner.clone(),
            token_id: token_id.clone(),
            price: U128(one_near),
        };

        let actual_sale: SaleForInserting = sale_for_contract.into();
        let expected_sale = SaleForInserting {
            prev_owner,
            curr_owner,
            token_id,
            price: Decimal::from(1),
        };
        assert_eq!(actual_sale, expected_sale);
    }
}
