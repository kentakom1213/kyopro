// https://onlinejudge.u-aizu.ac.jp/beta/room.html#TUPC2021/problems/A

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        M: usize,
        mut s0: Chars,
        mut s1: Chars,
        mut tt: [(Chars, Chars); M],
    }

    // 文字列をそれぞれソート
    s0.sort();
    s1.sort();

    for (t1, t2) in tt.iter_mut() {
        t1.sort();
        t2.sort();
    }

    // (a~z, a~z)を試す
    for c in 'a'..'z' {
        for d in 'a'..'z' {
            match (
                s0.binary_search(&c),
                s1.binary_search(&d),
            ) {
                (Ok(_), Ok(_)) => {
                    let mut isOK = true;

                    for (t1, t2) in tt.iter() {
                        isOK &= match (
                            t1.binary_search(&c),
                            t2.binary_search(&d),
                        ) {
                            (Ok(_), Ok(_)) => false,
                            _ => true,
                        };
                    }

                    if isOK {
                        println!("Yes\n{}{}", c, d);
                        return;
                    }
                },
                _ => { continue; }
            };
        }
    }

    println!("No");
}