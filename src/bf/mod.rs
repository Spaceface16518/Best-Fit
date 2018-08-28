use std::cmp::PartialOrd;

pub fn mean<T>(list: &[T]) -> T where T: PartialOrd {
    list.iter().sum() / list.len()
}

pub fn bf_slope<T>(x: &[T], y: &[T]) -> T where T: PartialOrd {
    let nums: &[(T, T)] = x.iter().zip(y.iter());
    let mean_x: T = mean(x);
    let mean_y: T = mean(y);
    nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        acc + (c.1 - mean_x) * (c.2 - mean_y)
    }) / nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        acc + util::pow(c.1 - mean_x, 2)
    })
}

pub fn y_intercept<T>(x: T, y: T, slope: T) -> T where T: PartialOrd {
    y - slope * x
}

pub mod util {
    pub fn pow<T>(base: T, exponent: T) -> T where T: PartialOrd {
        if exponent == 0 {
            return 1 as T;
        } else if base & 1 == 1 {
            return base * pow(base, exponent - { 1 as T });
        } else {
            let p = pow(base, exponent / { 2 as T });
            return p * p;
        }
    }
}
