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
    fn factor(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.sieve[x]);
            x /= self.sieve[x];
        }
        factors
    }
}
