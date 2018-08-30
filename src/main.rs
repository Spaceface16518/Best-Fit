mod bf;

fn main() {
    let x: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let y: [i32; 10] = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20];
    let slope: i32 = bf::bf_slope(&x, &y);
    let y_intercept: i32 = bf::y_intercept(bf::mean::mean(&x), bf::mean::mean(&y), slope);
    println!("x: {:?} y: {:?} slope: {} y-intercept: {}", &x, &y, slope, y_intercept);
}
