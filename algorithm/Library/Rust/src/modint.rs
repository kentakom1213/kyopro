#![allow(dead_code)]

const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize{
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_madd() {
        let x: usize = 998244355;
        let y: usize = 998244359;
        assert_eq!(x.madd(y), 8);

        let a: usize = 998244353;
        let b: usize = 1000000007;
        let c: usize = 20021213;
        assert_eq!(a.madd(b).madd(c), 21776867);
    }

    #[test]
    fn test_mneg() {
        let x: usize = 0;
        assert_eq!(x.mneg(), 0);
    }

    #[test]
    fn test_msub() {
        let x: usize = 0;
        let y: usize = 1000000007;
        assert_eq!(x.msub(y), 996488699);

        let a: usize = 288230376151711744;   // 1 << 58
        let b: usize = 576460752303423488;   // 1 << 59
        let c: usize = 1152921504606846976;  // 1 << 60
        assert_eq!(a.mneg().msub(b).msub(c), 553154679);
    }

    #[test]
    fn test_mpow() {
        let x: usize = 2;
        let y: usize = 1000000007;
        assert_eq!(x.mpow(y), 132727571);

        let a: usize = 998244353;
        let b: usize = 1024;
        assert_eq!(a.mpow(b), 0);
    }

    #[test]
    fn test_minv() {
        assert_eq!(1.minv(), 1);
    }

    #[test]
    #[should_panic]
    fn test_minv_err() {
        0.minv();
    }

    #[test]
    #[should_panic]
    fn test_mdiv_err() {
        1.mdiv(0);
    }
}