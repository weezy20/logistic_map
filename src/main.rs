fn main() {
    let x = 0.4_f64;
    let mut x_next = x;
    for i in 0..100 {
        println!("{}: {}", i, x_next);
        x_next = next(x_next);
    }
}
// x in [0,1]
fn next(x: f64) -> f64 {
    // 1 < k < 4
    let k = 1.1;
    // x(n+1) = k.x(n).[1 - x(n)]
    k * x * (1.0 - x)
}
