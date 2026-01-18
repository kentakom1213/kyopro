#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::factorize::factorize_pairs};
use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

/// n の約数の個数 f(n) は
///
/// n = b1^p1 ⋅⋯⋅ bk^pk
///
/// と表されるとき，
///
/// f(n) = (p1 + 1)⋯(pk + 1).
///
/// ---
///
/// 75 = 3⋅5^2 より，
///
/// - 75 = 74 + 1
/// - 3⋅25 = (2 + 1)(24 + 1)
/// - 5⋅15 = (4 + 1)(14 + 1)
/// - 3⋅5⋅5 = (2 + 1)(4 + 1)(4 + 1)
///
/// 指数の列が (74),(2,24),(4,14),(2,4,4) になるような N! の約数を数えれば良い．
///
/// ---
fn main() {
    input! {
        N: usize,
    }

    // 素因数分解
    let factors = {
        let mut fs: FxHashMap<usize, usize> = FxHashMap::default();
        for n in 1..=N {
            for (b, p) in factorize_pairs(n) {
                *fs.entry(b).or_default() += p;
            }
        }
        fs.into_iter().sorted().collect::<Vec<_>>()
    };
    debug!(factors);

    let mut ans = 0;

    // 75
    for &(a, p) in &factors {
        if p >= 74 {
            debug!((a, p));
            ans += 1;
        }
    }

    // 3⋅25
    for &(a, p) in &factors {
        for &(b, q) in &factors {
            if a != b && p >= 2 && q >= 24 {
                debug!((a, p), (b, q));
                ans += 1;
            }
        }
    }

    // 5⋅15
    for &(a, p) in &factors {
        for &(b, q) in &factors {
            if a != b && p >= 4 && q >= 14 {
                debug!((a, p), (b, q));
                ans += 1;
            }
        }
    }

    // 3⋅5⋅5
    for &(a, p) in &factors {
        for &(b, q) in &factors {
            for &(c, r) in &factors {
                if a != b && a != c && b < c && p >= 2 && q >= 4 && r >= 4 {
                    debug!((a, p), (b, q), (c, r));
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
