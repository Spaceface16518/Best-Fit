use std::ops;
use super::num;

pub fn mean<T>(list: &[T]) -> T
    where T: num::Num + ops::Add<T, Output=T> + ToOwned + Into<usize> {
    let sum = {
        let mut sum = T::zero();
        for i in list.iter() {
            sum = sum + i.to_owned();
        }
        sum
    };
    sum / T::into(list.len())
}