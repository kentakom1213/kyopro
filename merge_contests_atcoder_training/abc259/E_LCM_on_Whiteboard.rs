//          E - LCM on Whiteboard
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc259/tasks/abc259_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

type Num = BTreeMap<usize, usize>;

// solve
fn main() {
    input! {N: usize}

    let nums: Vec<Num> = (0..N)
        .map(|_| {
            input! {m: usize}
            let mut res: Num = BTreeMap::new();
            for _ in 0..m {
                input!{p: usize, e: usize}
                res.insert(p, e);
            }
            res
        })
        .collect();

    // L[素因数] = (指数の最大値, 最大値を達成するnumの数)
    let mut L: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for num in &nums {
        for (p, &e) in num.iter() {
            match L.get(p) {
                Some(_) => {
                    let (v, cnt) = L.get_mut(p).unwrap();
                    if *v == e {
                        *cnt += 1;
                    }
                    else if *v < e {
                        *v = e;
                        *cnt = 1;
                    }
                },
                None => {
                    L.insert(*p, (e, 1));
                },
            }
        }
    }

    // 各numについて、それを消した時に別の最大公約数が生まれるかを考える
    let mut c = 0;
    for num in &nums {
        let mut tmp = false;
        for (p, &e) in num {
            if let Some(&(v, cnt)) = L.get(p) {
                tmp |= e == v && cnt == 1;
            }
        }
        if tmp {
            c += 1;
        }
    }

    let ans = N.min(c + 1);  // Li == L が存在する場合を考える
    println!("{}", ans);
}
