/// ## 非再帰拡張Euclid互除法
/// ax + by = 1を満たすxを高速に求める
/// - **引数**: `a`,`m`
/// - **戻り値**: `a^{-1} mod m`
fn minv(mut a: usize, m: usize) -> usize {
    let mut b = m;
    let (mut u, mut v) = (1, 0);
    while b > 0 {
        let t = a / b;
        a -= t * b;
        u -= t * v;
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 { u += m };
    u
}

fn main() {
    let m = 998_244_353;

    let a = 2;
    println!("{}^-1 mod {} = {}", a, m, minv(a, m));

    let b = 1000;
    println!("{}^-1 mod {} = {}", b, m, minv(b, m));
}
