#![allow(dead_code)]

/// 余りをとる累乗
fn powmod(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_powmod_998244353() {
        const M: usize = 998244353;
        assert_eq!(powmod(2, 40, M), 444595123);
        assert_eq!(powmod(3, 20, M), 492051342);
        assert_eq!(powmod(2, M-2, M), 499122177);
        assert_eq!(powmod(M-1, M-2, M), M-1);
    }

    #[test]
    fn test_powmod_1048576() {
        const M: usize = 1048576;
        assert_eq!(powmod(2, 40, M), 0);
        assert_eq!(powmod(3, 20, M), 269201);
        assert_eq!(powmod(2, M-2, M), 0);
        assert_eq!(powmod(M-1, M-2, M), 1);
    }
}
