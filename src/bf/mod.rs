extern crate num;

use self::num::Num;
use std::cmp::PartialOrd;

pub mod util;

pub fn bf_slope<T>(x: &[T], y: &[T]) -> T where T: Num {
    let nums: &[(T, T)] = {
        let mut n: Vec<(T, T)> = Vec::new();
        for (i, cp) in x.iter().zip(y.iter()).enumerate() {
            let val = (*cp.0, *cp.1);
            n.push(val);
        }
        &n[..]
    };
    let mean_x: T = util::mean(x);
    let mean_y: T = util::mean(y);
    // Slope of best fit formula
    nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        acc + (c.0 - mean_x) * (c.1 - mean_y)
    }) / x.iter().fold(0, |acc: T, c: T| -> T {
        acc + util::pow(c - mean_x, 2)
    })
}

pub fn y_intercept<T>(x: T, y: T, slope: T) -> T where T: Num {
    y - slope * x
}
