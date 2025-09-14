use anchor_lang::prelude::*;
pub fn price_to_sqrt_price_x64(price: u64) -> Result<u128> {
    // Convert u64 price to f64 for sqrt calculation
    let price_f64 = price as f64;
    let sqrt_price = price_f64.sqrt();

    // Convert to Q64.64 fixed point
    let sqrt_price_x64 = (sqrt_price * (1u128 << 64) as f64) as u128;

    Ok(sqrt_price_x64)
}

pub fn sqrt_price_x64_to_tick(sqrt_price_x64: u128) -> Result<i32> {
    // Convert back to f64
    let sqrt_price_f64 = sqrt_price_x64 as f64 / (1u128 << 64) as f64;
    let price = sqrt_price_f64 * sqrt_price_f64; // square it

    // log base 1.0001
    let tick = price.log(1.0001);

    Ok(tick.floor() as i32)
}
