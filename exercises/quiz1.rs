// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!

// Put your function here!
fn calculate_price_of_apples(quantity: u32) -> u32 {
    let price = if quantity <= 40 { 2 } else { 1 };
    quantity * price
}

// Don't modify this function!
#[test]
fn verify_test() {
    let test_cases = [(35, 70), (40, 80), (41, 41), (65, 65)];
    for (quantity, expected_price) in test_cases {
        let got = calculate_price_of_apples(quantity);
        assert_eq!(expected_price, got)
    }
}
