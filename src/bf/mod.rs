extern crate num;

use std::ops;

pub mod mean;
pub mod pow;

pub fn bf_slope<T>(x: &[T], y: &[T]) -> T
    where T: num::Num + ops::Mul<T, Output=T> + ops::Add<T, Output=T> + ops::Sub<T, Output=T> + ops::BitAnd<T, Output=T> + Clone {
    let nums: &[(T, T)] = {
        let mut n: Vec<(T, T)> = Vec::new();
        for (i, cp) in x.iter().zip(y.iter()).enumerate() {
            let val = (*cp.0, *cp.1);
            n.push(val);
        }
        &n[..]
    };
    let mean_x: T = mean::mean(x);
    let mean_y: T = mean::mean(y);
    // Slope of best fit formula
    nums.iter().fold(T::zero(), |acc, c| -> T {
        acc + (c.0 - mean_x) * (c.1 - mean_y)
    }) / x.iter().fold(T::zero(), |acc, c| -> T {
        acc + pow::pow(c.clone() - mean_x, 2)
    })
}

pub fn y_intercept<T>(x: T, y: T, slope: T) -> T
    where T: num::Num + ops::Sub<T, Output=T> + ops::Mul<T, Output=T> {
    y - slope * x
}
