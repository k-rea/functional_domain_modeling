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

enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

struct WidgetCode(String);
struct GizmoCode(String);

enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity)
}
struct UnitQuantity(i32);
struct KilogramQuantity(f64);


struct PersonalName {
    first_name: String50,
    middle_initial: Option<String1>,
    last_name: String50,
}

struct String50(String);
struct String1(char);

struct EmailAddress(String);


fn main() {
    println!("Hello, functional domain modeling!");
}
