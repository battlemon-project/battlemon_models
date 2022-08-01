#[mixin::declare]
#[derive(Debug, Clone, PartialEq)]
pub struct SaleAbstract {
    pub prev_owner: String,
    pub curr_owner: String,
    pub token_id: String,
}
