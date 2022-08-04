use near_sdk::json_types::U128;

#[mixin::insert(SaleAbstract)]
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SaleForContract {
    pub price: U128,
}
