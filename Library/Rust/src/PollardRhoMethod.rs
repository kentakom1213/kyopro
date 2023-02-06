#[allow(dead_code)]

use crate::MillerRabinTest::is_prime_MR;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ## pollard_rho
/// ポラード・ロー法を適用し、約数を見つける
fn pollard_rho(N: usize) -> usize {
    if N % 2 == 0 { 
        return 2;
    }
    if is_prime_MR(N) {
        return N;
    }
    let f = |x: usize| -> usize {
        (((x as u128).pow(2) + 1) % N as u128) as usize
    };
    let mut step = 0;
    loop {
        step += 1;
        let mut x = step;
        let mut y = f(x);
        loop {
            let p = gcd(N + y - x, N);
            if p == 0 || p == N { break; }
            if p != 1 { return p; }
            x = f(x);
            y = f(f(y));
        }
    }
}

/// ## factorize
/// ポラード・ロー法による高速素因数分解
/// `O(n^(1/4))`
fn factorize(N: usize) -> Vec<usize> {
    if N == 1 { return vec![]; }
    let p = pollard_rho(N);
    if p == N { return vec![N]; }
    let mut left = factorize(p);
    let mut right = factorize(N / p);
    left.append(&mut right);
    left.sort();
    left
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_algo_method() {
        assert_eq!(factorize(4033), vec![37, 109]);
        assert_eq!(factorize(4681), vec![31, 151]);
        assert_eq!(factorize(1000000007), vec![1000000007]);
        assert_eq!(factorize(9999999999999), vec![3, 3, 53, 79, 265371653]);
        assert_eq!(factorize(341550054645379), vec![341550054645379]);
        assert_eq!(factorize(347484690041206937), vec![381727069, 910296173]);
    }
}
