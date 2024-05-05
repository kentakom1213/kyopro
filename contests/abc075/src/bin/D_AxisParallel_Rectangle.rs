//       D - Axis-Parallel Rectangle       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc075/tasks/abc075_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 点の全探索
///   - comb(50, 25) = 126410606437752 > 10^14
///   - → 無理
/// - 領域の全探索
fn main() { 
    input! {
        (N, K): (usize, usize),
        XY: [(isize, isize); N],
    }

    let mut xs: Vec<isize> = XY.iter().map(|v| v.0).collect();
    let mut ys: Vec<isize> = XY.iter().map(|v| v.1).collect();
    xs.sort();
    ys.sort();
    
    let mut ans = (xs[N-1] - xs[0]) * (ys[N-1] - ys[0]);

    for xi in 0..N {
        for xj in xi+1..N {
            for yi in 0..N {
                for yj in yi+1..N {

                    let (l, r) = (xs[xi], xs[xj]);
                    let (b, t) = (ys[yi], ys[yj]);
                    let area = (r - l) * (t - b);

                    // 領域に含まれる点の数
                    let mut cnt = 0;
                    for &(x, y) in &XY {
                        if l <= x && x <= r && b <= y && y <= t {
                            cnt += 1;
                        }
                    }

                    if cnt >= K {
                        chmin!(
                            ans,
                            area,
                        );
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
