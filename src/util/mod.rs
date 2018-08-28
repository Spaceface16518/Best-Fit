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
