//            E - Amusement Park           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc216/tasks/abc216_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

/// ## RunLengthEncode
/// ランレングス圧縮
fn run_length_encode<T>(arr: &[T]) -> Vec<(T, usize)>
where T: PartialEq + Copy
{
    let mut res = vec![];
    let mut cur = arr[0];
    let mut cnt = 1;
    for &val in &arr[1..] {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cur, cnt));
            cur = val;
            cnt = 1;
        }
    }
    let last_elem = *arr.last().unwrap();
    res.push((last_elem, cnt));

    res
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 5_000_000_000;
const NEG1: usize = 1_usize.wrapping_neg();

/// 初項a, 末項b, 公差1の等差数列の和
fn sum(a: usize, b: usize) -> usize {
    (a + b) * (b - a + 1) / 2
}

// main
fn main() {
    input! {
        N: usize,
        mut K: usize,
        mut A: [usize; N],
    }

    // 番兵を追加
    A.push(0);

    // ソート
    A.sort();

    // ランレングス圧縮
    let mut vals = run_length_encode(&A);

    let mut ans = 0;

    while K > 0 && vals.len() >= 2 {
        debug!(ans, &vals);

        let (fst, cnt1) = vals.pop().unwrap();
        let (snd, cnt2) = vals.pop().unwrap();
        // fst == sndとなるまでansに追加する
        // (fst - snd) * cnt1 回加算できる
        if (fst - snd) * cnt1 <= K {
            // (snd+1, snd+2, ..., fst の和) * cnt1
            ans += sum(snd + 1, fst) * cnt1;
            vals.push((snd, cnt1 + cnt2));
            K -= (fst - snd) * cnt1;
        }
        else {
            // 何回取れるか
            let quot = K / cnt1;
            ans += sum(fst - quot + 1, fst) * cnt1;
            // あまりを考える
            let rem = K % cnt1;
            ans += (fst - quot) * rem;
            K = 0;
        }
    }

    println!("{}", ans);
}