#[mixin::declare]
#[derive(Debug, Clone, PartialEq)]
pub struct SaleAbstract {
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
}

#[cfg(feature = "market")]
use chrono::Utc;
#[cfg(feature = "market")]
use rust_decimal::Decimal;
#[cfg(feature = "market")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "market")]
#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaleForRest {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "market")]
#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaleForDb {
    pub id: i64,
    pub date: chrono::DateTime<Utc>,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "market-contract")]
use near_sdk::json_types::U128;

#[cfg(feature = "market-contract")]
#[mixin::insert(SaleAbstract)]
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleForContract {
    pub price: U128,
}

#[cfg(feature = "faker")]
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
#[cfg(feature = "faker")]
use fake::{locales::EN, Dummy, Fake, Faker};
#[cfg(feature = "faker")]
use rand::Rng;

#[cfg(feature = "faker")]
impl Dummy<Faker> for SaleForRest {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let scale = rng.gen_range(0..=24);
        let lo = rng.gen();
        let mid = rng.gen();
        let price = Decimal::from_parts(lo, mid, 0, false, scale);
        Self {
            prev_owner: format!("{}.near", Word().fake::<String>()),
            curr_owner: format!("{}.near", Word().fake::<String>()),
            token_id: NumberWithFormat(EN, "^########").fake::<String>(),
            price,
        }
    }
}

#[cfg(feature = "faker")]
impl Dummy<Faker> for SaleForDb {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
        let sale_without_id: SaleForRest = Faker.fake();
        Self {
            id: Faker.fake(),
            date: DateTime().fake(),
            prev_owner: sale_without_id.prev_owner,
            curr_owner: sale_without_id.curr_owner,
            token_id: sale_without_id.token_id,
            price: sale_without_id.price,
        }
    }
}

#[cfg(feature = "market-convert")]
use crate::market::convert_to_decimal_near;

#[cfg(feature = "market-convert")]
impl From<SaleForContract> for crate::market::sale::SaleForRest {
    fn from(sale: SaleForContract) -> Self {
        let price = convert_to_decimal_near(&sale.price);

        Self {
            prev_owner: sale.prev_owner,
            curr_owner: sale.curr_owner,
            token_id: sale.token_id,
            price,
        }
    }
}
