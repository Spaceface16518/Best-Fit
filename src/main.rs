extern crate num;

mod bf;

fn main() {
    {
        let base = 5;
        let exp = 3;
        println!("base: {}, exponent: {}, result with custom: {}, result with standard: {}", base, exp, bf::pow::exper_pow(base, exp), num::pow::pow(base.clone(), exp.clone()));
    }
    {
        let nums = [1f64, 2f64, 3f64, 4f64, 5f64];
        let avg: i32 = bf::mean::mean(&nums[..]) as i32;
        println!("The avg of {:?} is {:?}", nums, avg)
    };
    {
        let x = [1f64, 2f64, 3f64, 4f64, 5f64];
        let y = [4f64, 6f64, 8f64, 10f64, 12f64];
        let mean_x = bf::mean::mean(&x);
        let mean_y = bf::mean::mean(&y);
        let slope = bf::bf_slope(&x, &y);
        let y_intercept = bf::y_intercept(mean_x, mean_y, slope);
        println!("x: {:?}, y: {:?}\nmean x: {}, mean y: {}\nslope: {} y-intercept: {}", &x, &y, mean_x, mean_y, slope, y_intercept);
    };
    let weight = [9.8, 7.35, 4.9, 2.45, 19.6, 17.15, 14.7, 12.25];
    let avg_k = [2.5, 2.3, 1.7, 1.2, 4.8, 4.4, 3.9, 2.7];
    let slope = bf::f_bf_slope(&weight, &avg_k);
    let y_intercept = bf::y_intercept(bf::mean::f_mean(&weight[..]), bf::mean::f_mean(&avg_k), slope);
    println!("weight: {:?} avg_k: {:?} slope: {} y-intercept: {}", &weight, &avg_k, slope, y_intercept);
}
