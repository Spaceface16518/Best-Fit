extern crate num;

use std::iter;
use std::ops;

pub mod mean;
pub mod pow;

pub fn bf_slope<T>(x: &[T], y: &[T]) -> T
                   where T: num::Num + ops::Mul<T, Output=T> + ops::Add<T, Output=T> + ops::Sub<T, Output=T> + Clone + From<f64> + iter::Sum {
    let nums = {
        let mut n: Vec<(T, T)> = Vec::new();
        for (cp_x, cp_y) in x.iter().zip(y.iter()) {
            let val = (cp_x.clone(), cp_y.clone());
            n.push(val);
        }
        &n.to_owned()[..]
    };
    let mean_x = mean::mean(x);
    let mean_y = mean::mean(y);
    // Slope of best fit formula
    nums.iter().fold(T::zero(), |acc, (x_i, y_i)| -> T {
        acc + (x_i.clone() - mean_x.clone()) * (y_i.clone() - mean_y.clone())
    }) / x.iter().fold(T::zero(), |acc, x_i| -> T {
        acc + pow::pow(x_i.clone() - mean_x.clone(), 2)
    })
}

pub fn y_intercept<T>(x: T, y: T, slope: T) -> T
                      where T: num::Num + ops::Sub<T, Output=T> + ops::Mul<T, Output=T> + From<f64> {
    y - slope * x
}
