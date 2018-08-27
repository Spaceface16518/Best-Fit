pub fn mean<T>(list: &[T]) -> T where T: PartialOrd {
    list.iter().sum() / list.len()
}

pub fn bf_slope<T>(nums: &[(T, T)], mean_x: T, mean_y: T) -> T where T: PartialOrd {
    nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        acc + (c.1 - mean_x) * (c.2 - mean_y)
    }) / nums.iter().fold(0, |acc: T, c: (T, T)| -> T {
        let cc: T = (c.1 - mean_x);
        acc + (cc * cc)
    })
}