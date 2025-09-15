use anchor_lang::prelude::*;
pub fn price_to_sqrt_price_x64(price: u64) -> Result<u128> {
    let price_f64 = price as f64;
    let sqrt_price = price_f64.sqrt();

    let sqrt_price_x64 = (sqrt_price * (1u128 << 64) as f64) as u128;

    Ok(sqrt_price_x64)
}

pub fn sqrt_price_x64_to_tick(sqrt_price_x64: u128) -> Result<i32> {
    let sqrt_price_f64 = sqrt_price_x64 as f64 / (1u128 << 64) as f64;
    let price = sqrt_price_f64 * sqrt_price_f64;

    let tick = price.log(1.0001);

    Ok(tick.floor() as i32)
}

pub fn integer_sqrt(value: u128) -> u128 {
    let mut z = value;
    let mut x = value / 2 + 1;
    while x < z {
        z = x;
        x = (value / x + x) / 2;
    }
    z
}
