#![allow(dead_code)]

struct Factors {
    n: usize,
    sieve: Vec<usize>,
}

impl Factors {
    fn new(n: usize) -> Self {
        let mut facs = Factors {
            n: n,
            sieve: vec![1; n+1],
        };
        for i in 2..=n {
            for j in 1.. {
                if i*j > n { break; }
                if facs.sieve[i*j] == 1 {
                    facs.sieve[i*j] = i;
                }
            }
        }
        facs
    }

    /// 素因数分解を高速（`O(logn)`）で行う
    fn factorize(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.sieve[x]);
            x /= self.sieve[x];
        }
        factors
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorize() {
        let f = Factors::new(200_000);

        assert_eq!(f.factorize(200), vec![2, 2, 2, 5, 5]);
        assert_eq!(f.factorize(123450), vec![2, 3, 5, 5, 823]);
        assert_eq!(f.factorize(107311), vec![239, 449]);
        assert_eq!(f.factorize(199999), vec![199999]);
    }
}
