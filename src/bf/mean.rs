use std::ops;
use super::num;

pub fn mean<T>(list: &[T]) -> T
    where T: num::Num + ops::Add<T, Output=T> {
    let sum: T = {
        let mut sum: T = T::zero();
        for &i in list.iter() {
            sum = sum + i;
        }
        sum
    };
    sum / list.len()
}