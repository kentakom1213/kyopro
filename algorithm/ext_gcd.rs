/// ## 拡張Euclid互除法
/// ax + by = 1を満たすxを高速に求める
/// - **引数**: `a`,`b`
/// - **戻り値**: `ax + by = gcd(a, b) = d`となる`(x,y,d)`
fn ext_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    println!("a:{}, b:{}", a, b);
    if b == 0 {
        return (1, 0, a);
    }
    let (q, r) = (a / b, a % b);
    let (x, y, d) = ext_gcd(b, r);
    let (s, t) = (y, x - q * y);
    (s, t, d)
}

fn main() {
    const MOD: isize = 998244353;
    let inv2 = ext_gcd(2, MOD);
    println!("{:?}", inv2);
}
