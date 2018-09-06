extern crate num;

use std::ops;

pub fn pow<T>(base: T, exponent: usize) -> T where T: Clone + num::Num + ops::Mul<T, Output=T> {
    let mut base: T = base.clone();
    let mut exponent: usize = exponent.clone();

    // Base and special cases
    if exponent == 0 {
        return T::one();
    }
    if exponent & 1 == 0 {
        base = base.clone() * base;
        exponent >>= 1;
    }
    if exponent == 1 {
        return base;
    }

    let mut acc: T = base.clone();

    while exponent > 1 {
        exponent >>= 1;
        base = base.clone() * base;
        if exponent & 1 == 1 {
            acc = base.clone() * acc;
        }
    }
    return acc;
}
