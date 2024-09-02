#[derive(Debug)]
pub struct UnvalidatedOrder {
    pub order_id: String,
}

#[derive(Debug)]
pub struct UnvalidatedOrderLine {
    pub product_code: String
}