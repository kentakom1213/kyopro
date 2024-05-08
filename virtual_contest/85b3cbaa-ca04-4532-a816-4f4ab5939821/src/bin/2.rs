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

use crate::enum_pairs::pairs_usize;

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
    }

    let mut A = vec![];

    for i in (1..=2 * N - 1).rev() {
        input! {
            a: [usize; i]
        }
        A.push(a);
    }

    debug2D!(A);

    let mut ans = 0;

    for ps in pairs_usize(2 * N) {
        debug!(ps);
        let cost = ps
            .iter()
            .map(|&(i, j)| A[i][j - i - 1])
            .fold(0, |x, y| x ^ y);

        chmax! {
            ans,
            cost
        };
    }

    println!("{ans}");
}

mod enum_pairs {
    #![allow(dead_code)]
    //! N人をペアに分ける組合せを全列挙する
    /// ペアを列挙する
    #[derive(Debug)]
    pub struct PairsIterator<T: Clone> {
        stack: Vec<(Vec<T>, Vec<(T, T)>)>,
    }
    impl<T: Clone> FromIterator<T> for PairsIterator<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self {
                stack: vec![(iter.into_iter().collect::<Vec<T>>(), vec![])],
            }
        }
    }
    impl<T: Clone> Iterator for PairsIterator<T> {
        type Item = Vec<(T, T)>;
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let Some((rem, pairs)) = self.stack.pop() else {
                    return None;
                };
                if rem.len() < 2 {
                    return Some(pairs);
                }
                for i in (1..rem.len()).rev() {
                    let mut new_rem = rem.clone();
                    let snd = new_rem.remove(i);
                    let fst = new_rem.remove(0);
                    let mut new_pairs = pairs.clone();
                    new_pairs.push((fst, snd));
                    // 新しい要素を追加
                    self.stack.push((new_rem, new_pairs));
                }
            }
        }
    }
    /// (0〜n-1)のn個の要素からなる系列
    /// をペアにする組合せを列挙する
    pub fn pairs_usize(n: usize) -> PairsIterator<usize> {
        (0..n).collect()
    }
}

mod macro_chmax {
    #![allow(dead_code)]
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
