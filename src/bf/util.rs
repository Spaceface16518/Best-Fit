use super::num::Num;

pub fn pow<T>(base: T, exponent: T) -> T where T: Num {
    if exponent == 0 {
        return 1 as T;
    } else if base & 1 == 1 {
        return base * pow(base, exponent - { 1 as T });
    } else {
        let p = pow(base, exponent / { 2 as T });
        return p * p;
    }
}

pub fn mean<T>(list: &[T]) -> T where T: Num {
    list.iter().sum()[..] as [T] / list.len()
}
