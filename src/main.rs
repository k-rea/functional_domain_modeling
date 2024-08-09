struct OrderId(String);
struct CustomerId(String);
struct ShippingAddress(String);
struct BillingAddress(String);

struct UnvalidatedOrder;
struct ValidatedOrder;
struct PricedOrder;

type ValidateOrder = fn(UnvalidatedOrder) -> ValidatedOrder;
type PriceOrder = fn(ValidatedOrder) -> PricedOrder;

struct Order {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
}

struct CustomerInfo {
    id: CustomerId,
    name: String,
    email: String,
}

struct OrderLine {
    product_code: String,
    quantity: i32,
    price: f64,
}

fn main() {
    println!("Hello, functional domain modeling!");
}
