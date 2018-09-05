extern crate num;

mod bf;

fn main() {
    let weight = [9.8, 7.35, 4.9, 2.45, 19.6, 17.15, 14.7, 12.25];
    let avg_k = [2.5, 2.3, 1.7, 1.2, 4.8, 4.4, 3.9, 2.7];
    let slope = bf::bf_slope(&weight, &avg_k);
    let y_intercept = bf::y_intercept(bf::mean::mean(&weight[..]), bf::mean::mean(&avg_k), slope);
    println!("weight: {:?} avg_k: {:?}\nslope: {} y-intercept: {}", &weight, &avg_k, slope, y_intercept);
}
