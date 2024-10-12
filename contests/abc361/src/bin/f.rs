#![allow(non_snake_case)]

use cp_library_rs::debug;
use num_integer::Roots;
use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
    }

    // k^2 <= N であるようなkの個数
    let mut ans = N.sqrt();

    debug!(ans);

    // すでに見たもの
    let mut set = FxHashSet::default();

    // a^b <= N であるようなb(>2)について全探索
    for b in 3_u32..=60 {
        let mut a = 2_usize;
        loop {
            let ab = a.saturating_pow(b);
            if ab > N {
                break;
            }
            // k^2 の形で表されるかを判定
            if ab.sqrt().pow(2) != ab && !set.contains(&ab) {
                ans += 1;
                set.insert(ab);
            }
            // debug!(a, b);
            a += 1;
        }
    }

    println!("{ans}");
}
