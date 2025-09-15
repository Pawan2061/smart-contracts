use crate::state::error::CLMMERROR;
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

pub fn swap_step(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    amount_remaining: u128,
    zero_for_one: bool,
) -> Result<(u128, u128, u128)> {
    require!(liquidity > 0, CLMMERROR::NoLiquidity);

    let mut amount_in_consumed: u128 = 0;
    let mut amount_out_produced: u128 = 0;
    let mut new_sqrt_price_x64 = sqrt_price_current_x64;

    if zero_for_one {
        let max_dx = get_amount_delta_x(liquidity, sqrt_price_target_x64, sqrt_price_current_x64)?;
        if amount_remaining < max_dx {
            new_sqrt_price_x64 = get_new_sqrt_price_from_input(
                liquidity,
                sqrt_price_current_x64,
                amount_remaining,
                true,
            )?;
            amount_in_consumed = amount_remaining;
            amount_out_produced =
                get_amount_delta_y(liquidity, new_sqrt_price_x64, sqrt_price_current_x64)?;
        } else {
            new_sqrt_price_x64 = sqrt_price_target_x64;
            amount_in_consumed = max_dx;
            amount_out_produced =
                get_amount_delta_y(liquidity, sqrt_price_target_x64, sqrt_price_current_x64)?;
        }
    } else {
        let max_dy = get_amount_delta_y(liquidity, sqrt_price_current_x64, sqrt_price_target_x64)?;
        if amount_remaining < max_dy {
            new_sqrt_price_x64 = get_new_sqrt_price_from_input(
                liquidity,
                sqrt_price_current_x64,
                amount_remaining,
                false,
            )?;
            amount_in_consumed = amount_remaining;
            amount_out_produced =
                get_amount_delta_x(liquidity, sqrt_price_current_x64, new_sqrt_price_x64)?;
        } else {
            new_sqrt_price_x64 = sqrt_price_target_x64;
            amount_in_consumed = max_dy;
            amount_out_produced =
                get_amount_delta_x(liquidity, sqrt_price_current_x64, sqrt_price_target_x64)?;
        }
    }

    Ok((amount_in_consumed, amount_out_produced, new_sqrt_price_x64))
}

fn get_amount_delta_x(liquidity: u128, sqrt_price_a: u128, sqrt_price_b: u128) -> Result<u128> {
    if sqrt_price_a > sqrt_price_b {
        return get_amount_delta_x(liquidity, sqrt_price_b, sqrt_price_a);
    }
    let numerator = liquidity << 64;
    let delta = numerator
        .checked_mul(sqrt_price_b - sqrt_price_a)
        .ok_or(CLMMERROR::MathOverflow)?
        / (sqrt_price_a
            .checked_mul(sqrt_price_b)
            .ok_or(CLMMERROR::MathOverflow)?
            >> 64);
    Ok(delta)
}

fn get_amount_delta_y(liquidity: u128, sqrt_price_a: u128, sqrt_price_b: u128) -> Result<u128> {
    if sqrt_price_a > sqrt_price_b {
        return get_amount_delta_y(liquidity, sqrt_price_b, sqrt_price_a);
    }
    let delta = liquidity
        .checked_mul(sqrt_price_b - sqrt_price_a)
        .ok_or(CLMMERROR::MathOverflow)?
        >> 64;
    Ok(delta)
}

fn get_new_sqrt_price_from_input(
    liquidity: u128,
    sqrt_price_current: u128,
    amount_in: u128,
    zero_for_one: bool,
) -> Result<u128> {
    if zero_for_one {
        let numerator = liquidity << 64;
        let product = amount_in
            .checked_mul(sqrt_price_current)
            .ok_or(CLMMERROR::MathOverflow)?;
        let denom = numerator
            .checked_add(product)
            .ok_or(CLMMERROR::MathOverflow)?;
        Ok((numerator
            .checked_mul(sqrt_price_current)
            .ok_or(CLMMERROR::MathOverflow)?
            / denom))
    } else {
        let delta = (amount_in << 64) / liquidity;
        Ok(sqrt_price_current
            .checked_add(delta)
            .ok_or(CLMMERROR::MathOverflow)?)
    }
}

pub fn tick_to_sqrt_price_x64(tick: i32) -> Result<u128> {
    let price = 1.0001_f64.powi(tick);
    let sqrt_price = price.sqrt();
    let sqrt_price_x64 = (sqrt_price * (1u128 << 64) as f64) as u128;
    Ok(sqrt_price_x64)
}
