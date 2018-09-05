use std::ops;
use super::num;

pub fn mean<T>(list: &[T]) -> T
               where T: num::Num + ops::Add<T, Output=T> + From<f64> + Clone {
    let sum: T = {
        // Perhaps replace with iterator? Though might be slower/inefficient
        let mut s_sum: T = T::zero();
        for i in 0..list.len() {
            s_sum = s_sum + list[i].clone();
        }
        s_sum
    };
    sum / T::from(list.len() as f64)
}

pub fn f_mean(list: &[f64]) -> f64 {
    let sum: f64 = list.iter().sum();
    return sum / list.len() as f64;
}
