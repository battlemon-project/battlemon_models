use rust_decimal::{Decimal, MathematicalOps};

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
