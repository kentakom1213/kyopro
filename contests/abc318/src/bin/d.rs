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

fn main() {
    input! {
        mut N: usize,
    }

    let mut D = vec![];

    for i in (1..N).rev() {
        input! {
            d: [usize; i]
        }
        D.push(d);
    }

    let mut G = vec![vec![0; N]; N];

    for i in 0..N {
        for j in i..N - 1 {
            debug!(i, j);
            G[i][j + 1] = D[i][j - i];
            G[j + 1][i] = D[i][j - i];
        }
    }

    // Nが奇数のとき，ダミーの頂点を追加
    if N % 2 == 1 {
        for i in 0..N {
            G[i].push(0);
        }
        N += 1;
        G.push(vec![0; N]);
    }

    debug!(G);

    let mut ans = 0;

    for pairs in pairs_usize(N) {
        let mut tmp = 0;
        for (a, b) in pairs {
            tmp += G[a][b];
        }
        chmax! {
            ans,
            tmp
        };
    }

    println!("{ans}");
}
