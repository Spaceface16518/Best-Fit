use std::cmp::PartialOrd;

pub fn mean<T>(list: &[T]) -> T where T: PartialOrd {
    list.iter().sum() / list.len()
}

pub fn bf_slope<T>(x: &[T], y: &[T], mean_x: T, mean_y: T) -> T where T: PartialOrd {
    let nums: &[(T, T)] = x.iter().zip(y.iter());
    nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        acc + (c.1 - mean_x) * (c.2 - mean_y)
    }) / nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        let cc: T = (c.1 - mean_x);
        acc + (cc * cc)
    })
}

pub fn y_intercept<T>(mean_x: T, mean_y: T, slope: T) -> T where T: PartialOrd {
    mean_y - slope * mean_x
}