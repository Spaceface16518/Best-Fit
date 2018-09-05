use std::ops;
use super::num;

pub fn mean<T>(list: &[T]) -> T
    where T: num::Num + ops::Add<T, Output=T> + From<usize> + Clone {
    let sum: T = {
        // Perhaps replace with iterator? Though might be slower/inefficient
        let mut s_sum: T = T::zero();
        for &i in list.iter() {
            s_sum = s_sum + i;
        }
        s_sum
    };
    sum / T::from(list.len())
}

pub fn f_mean(list: &[f64]) -> f64 {
    let sum: f64 = list.iter().sum();
    return sum / list.len() as f64;
}