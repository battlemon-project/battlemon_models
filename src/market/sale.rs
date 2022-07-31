use chrono::Utc;
use fake::faker::{chrono::en::DateTime, lorem::en::Word, number::raw::NumberWithFormat};
use fake::{locales::EN, Dummy, Fake, Faker};
use near_sdk::json_types::U128;
use rand::Rng;
use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

#[mixin::declare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaleAbstract {
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
}

#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaleForContract {
    pub price: U128,
}

#[mixin::insert(SaleAbstract)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaleForInserting {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}

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

impl From<SaleForContract> for SaleForInserting {
    fn from(sale: SaleForContract) -> Self {
        let price = Decimal::from(sale.price.0) / Decimal::new(10, 0).powu(24);

        Self {
            prev_owner: sale.prev_owner,
            curr_owner: sale.curr_owner,
            token_id: sale.token_id,
            price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_from_sale_for_contract_to_sale_for_inserting() {
        let one_near = 1_000_000_000_000_000_000_000_000;
        let prev_owner = "alice.near".to_string();
        let curr_owner = "bob.near".to_string();
        let token_id = "123456789".to_string();
        let sale_for_contract = SaleForContract {
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
