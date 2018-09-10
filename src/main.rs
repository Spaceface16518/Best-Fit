mod bf;

fn main() {
    let weight = [9.8, 7.35, 4.9, 2.45, 19.6, 17.15, 14.7, 12.25];
    let avg_k = [2.5, 2.3, 1.7, 1.2, 4.8, 4.4, 3.9, 2.7];
    let avg_s = [3.9, 3.6, 2.6, 1.7, 6.7, 7.3, 5.3, 4.4];
    {
        let slope = bf::bf_slope(&weight, &avg_k);
        let y_intercept = bf::y_intercept(bf::mean::mean(&weight[..]), bf::mean::mean(&avg_k), slope);
        println!("Kinetic:\n weight: {:?} avg_k: {:?}\n slope: {} y-intercept: {}", &weight, &avg_k, slope, y_intercept);
    }
    {
        let slope = bf::bf_slope(&weight, &avg_s);
        let y_intercept = bf::y_intercept(bf::mean::mean(&weight[..]), bf::mean::mean(&avg_s), slope);
        println!("Static:\n weight: {:?} avg_s: {:?}\n slope: {} y-intercept: {}", &weight, &avg_s, slope, y_intercept);
    }
}
