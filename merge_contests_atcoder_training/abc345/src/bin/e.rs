#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

use crate::run_length::{run_length_encode, run_length_encode_from};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: usize,
        CV: [(Usize1, usize); N],
    }

    let (C, V): (Vec<usize>, Vec<usize>) = CV.iter().cloned().unzip();
    let rle = run_length_encode(&C);
    debug!(rle);

    // 残すボールの価値の総和の最大値
    // <=> S - 取り除くボールの価値の総和の最小値

    // 必ず取り除くべきボールを消す
    let mut del_cnt = 0; // 消したボールの数
    let mut idx = 0;

    let mut rem_CV = vec![]; // 残った値

    for &(c, n) in &rle {
        // 最小値
        let mut max = 0;
        for _ in 0..n {
            chmax! {
                max,
                V[idx],
            };
            idx += 1;
        }
        // 取り除くべきボールの最小値
        del_cnt += n - 1;
        // 残った列
        rem_CV.push((c, max));
    }

    // 達成できない場合
    if del_cnt > K {
        println!("-1");
        return;
    }

    debug!(del_cnt);
    debug!(rem_CV);

    // 残りのボールから，隣あう同じ色ができないように削除していく
    // K <= 500 より，この操作を愚直に行ってもOK
    for _ in 0..K - del_cnt {
        // 候補: 左右の色が異なるボール
        let mut min_idx = 0;
        let mut min_val = INF;
        let L = rem_CV.len();
        for i in 0..L {
            if 0 < i && i < L - 1 && rem_CV[i - 1].0 == rem_CV[i + 1].0 {
                continue;
            }
            if chmin! {
                min_val,
                rem_CV[i].1
            } {
                min_idx = i;
            }
        }
        // 最小の候補を削除する
        rem_CV.remove(min_idx);
        debug!(rem_CV);
    }

    let ans = rem_CV.iter().map(|&(_, v)| v).sum::<usize>();

    println!("{ans}");
}

mod chmin {
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
}

mod chmax {
    //! chmaxの実装
    /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmax {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmax! {
                $a,
                ($b).max($c)
                $(,$other)*
            }
        }}
    }
}

mod run_length {
    //! ランレングス圧縮
    /// ## ランレングス圧縮
    /// - スライスからエンコードを行う
    pub fn run_length_encode<T>(arr: &[T]) -> Vec<(T, usize)>
    where
        T: PartialEq + Copy,
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
    /// ## ランレングス圧縮 (from Iterator)
    /// - イテレータからエンコードを行う
    pub fn run_length_encode_from<T, I>(mut itr: I) -> Vec<(T, usize)>
    where
        T: PartialEq,
        I: Iterator<Item = T>,
    {
        let mut res = vec![];
        let mut cur = itr.next().unwrap();
        let mut cnt = 1;
        for val in itr {
            if val == cur {
                cnt += 1;
            } else {
                res.push((cur, cnt));
                cur = val;
                cnt = 1;
            }
        }
        res.push((cur, cnt));
        res
    }
}
