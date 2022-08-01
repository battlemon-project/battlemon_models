use crate::market::sale_abstract::SaleAbstract;
use chrono::Utc;
use rust_decimal::Decimal;

#[cfg(feature = "faker")]
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
#[cfg(feature = "faker")]
use fake::{locales::EN, Dummy, Fake, Faker};
#[cfg(feature = "faker")]
use rand::Rng;

use serde::{Deserialize, Serialize};

#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaleForInserting {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "faker")]
impl Dummy<Faker> for SaleForInserting {
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

#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sale {
    pub id: i64,
    pub date: chrono::DateTime<Utc>,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

#[cfg(feature = "faker")]
impl Dummy<Faker> for Sale {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
        let sale_without_id: SaleForInserting = Faker.fake();
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
