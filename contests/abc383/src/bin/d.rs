#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::{data_structure::segmented_sieve::segmented_sieve, debug};
use proconio::input;

fn main() {
    input! {
        N: usize
    }

    // let mut set = BTreeSet::default();
    // for i in 1.. {
    //     if i * i > N {
    //         break;
    //     }
    //     if N % i == 0 {
    //         set.insert(i);
    //         set.insert(N / i);
    //     }
    // }
    // debug!(set);

    let primes = segmented_sieve(0, 2020202);

    debug!(&primes[..100]);

    let M = primes.len();

    let mut ans = 0;

    for i in 0..M {
        let p = primes[i];
        if p.saturating_mul(p) > N {
            break;
        }
        for j in i + 1..M {
            let q = primes[j];

            if p.saturating_mul(p).saturating_mul(q).saturating_mul(q) > N {
                break;
            }

            // debug!(p, q);
            ans += 1;
        }
    }

    // べき乗
    for i in 0.. {
        let p = primes[i];
        let p2 = p.saturating_mul(p);
        let p4 = p2.saturating_mul(p2);
        let p8 = p4.saturating_mul(p4);

        if p8 <= N {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{ans}");
}
