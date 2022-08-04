#[mixin::declare]
#[derive(Debug, Clone, PartialEq)]
struct BidAbstract {
    pub id: String,
}

#[cfg(feature = "market")]
use chrono::DateTime;
#[cfg(feature = "market")]
use chrono::Utc;
#[cfg(feature = "market")]
use rust_decimal::Decimal;

#[cfg(feature = "market")]
#[mixin::declare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct BidAbstractForRestAndDb {
    pub id: String,
    pub token_id: String,
    pub account_id: String,
    pub expire_at: Option<DateTime<Utc>>,
    pub create_at: DateTime<Utc>,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "market")]
#[mixin::insert(BidAbstractForRestAndDb)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidForRest {}

#[cfg(feature = "market")]
#[mixin::insert(BidAbstractForRestAndDb)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidForDb {}

#[cfg(feature = "faker")]
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
#[cfg(feature = "faker")]
use fake::{locales::EN, Dummy, Fake, Faker};
#[cfg(feature = "faker")]
use rand::Rng;
//
// impl Dummy<Faker> for BidForRest {
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
// impl Dummy<Faker> for BidForDb {
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
use near_sdk::serde::{Deserialize, Serialize};
#[cfg(feature = "market-contract")]
use near_sdk::{env, AccountId};

#[cfg(feature = "market-contract")]
#[mixin::insert(BidAbstract)]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct BidForContract {
    pub token_id: TokenId,
    pub expire_at: Option<u64>,
    pub account_id: AccountId,
    pub price: U128,
    pub create_at: u64,
}

#[cfg(feature = "market-contract")]
impl BidForContract {
    pub fn new(token_id: TokenId, expire_at: Option<u64>) -> Self {
        Self {
            id: crate::helpers_contract::generate_id(),
            token_id,
            expire_at,
            account_id: env::predecessor_account_id(),
            price: U128(env::attached_deposit()),
            create_at: env::block_timestamp(),
        }
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn price(&self) -> u128 {
        self.price.0
    }

    pub fn token_id(&self) -> &TokenId {
        &self.token_id
    }

    pub fn create_at(&self) -> u64 {
        self.create_at
    }
}

#[cfg(feature = "market-convert")]
impl From<BidForContract> for crate::market::bid::BidForRest {
    fn from(bid: BidForContract) -> Self {
        let price = crate::market::convert_to_decimal_near(&bid.price);
        let expire_at = bid
            .expire_at
            .map(|expire_at| crate::market::convert_from_near_timestamp(expire_at));
        let create_at = crate::market::convert_from_near_timestamp(bid.create_at);
        Self {
            id: bid.id,
            token_id: bid.token_id.to_string(),
            account_id: bid.account_id.to_string(),
            expire_at,
            create_at,
            price,
        }
    }
}
