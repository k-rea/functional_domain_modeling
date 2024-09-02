use crate::domain::common::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OrderId(String);

impl OrderId {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            Err("OrderId must not be empty".into())
        } else if value.len() > 50 {
            Err("OrderId must not be more thnan 50 chars".into())
        } else {
            Ok(OrderId(value))
        }
    }
}
