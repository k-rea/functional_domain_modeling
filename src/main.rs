struct OrderId(String);
struct CustomerId(String);
struct ShippingAddress(String);
struct BillingAddress(String);

struct UnvalidatedOrder;
struct ValidatedOrder;
struct PricedOrder;

type ValidateOrder = fn(UnvalidatedOrder) -> ValidatedOrder;
type PriceOrder = fn(ValidatedOrder) -> PricedOrder;

fn main() {
    println!("Hello, functional domain modeling!");
}
