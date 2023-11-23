// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize
    }

    // 2 * 10^6までの素数を全列挙
    let primes = {
        let mut sieve = vec![true; 1010101];
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..1010101 {
            for j in 2.. {
                if i * j >= 1010101 {
                    break;
                }
                sieve[i * j] = false;
            }
        }
        sieve
            .iter()
            .enumerate()
            .filter(|&(i, &x)| x)
            .map(|(x, _)| x)
            .collect_vec()
    };

    debug!(&primes[..20]);

    // 素数の2乗
    let primes2 = primes.iter().map(|x| x * x).collect_vec();

    let mut ans = 0;

    for i in 0..primes.len() {
        let a = primes2[i];
        for j in i + 1..primes.len() {
            let b = primes[j];
            if a * b > N {
                break;
            }
            for k in j + 1..primes.len() {
                let c = primes2[k];
                if a.saturating_mul(b).saturating_mul(c) > N {
                    break;
                }
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
