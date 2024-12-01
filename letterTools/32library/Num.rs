/*
数值特征和类型的集合。包括数字、大整数、复数等的泛型。
*/

use anyhow::{anyhow, Result};
use num::*;
use std::fmt::Display;

fn bounds_to_string<N: Bounded + Display>(number: N) -> String {
    format!(
        "value {} min is {} max is {}",
        number,
        N::min_value(),
        N::max_value()
    )
}

#[test]
fn bounds() {
    assert_eq!(bounds_to_string(12u8), "value 12 min is 0 max is 255");
    assert_eq!(
        bounds_to_string(33i16),
        "value 33 min is -32768 max is 32767"
    );
}

fn num_operations<N: Num>(a: &str, b: N) -> Result<N> {
    let a = N::from_str_radix(a, 10).map_err(|_| anyhow!("could not conert value"))?;
    let value = a + b - N::one();
    Ok(if value.is_zero() {
        value
    } else {
        value * (N::one() + N::one())
    })
}

#[test]
fn test_num_operations() -> Result<()> {
    assert_eq!(num_operations("2", 10i32)?, 22i32);
    assert_eq!(num_operations("-5", 6i8)?, 0i8);
    Ok(())
}

#[test]
fn greatest_common_divisor() -> Result<()> {
    assert_eq!(num::integer::gcd(25u8, 15u8), 5);
    assert_eq!(num::integer::gcd(1024i32, 65536i32), 1024);
    Ok(())
}