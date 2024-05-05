//                C - Sentou
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc060/tasks/arc073_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// # 座標圧縮
#[derive(Debug)]
pub struct Compression<'a, T> {
    pub size: usize,
    pub sorted_array: Vec<&'a T>,
}

impl<'a, T: Ord> Compression<'a, T> {
    pub fn new(array: &'a [T]) -> Self {
        let mut comp: Vec<&T> = array.iter().collect();
        comp.sort();
        comp.dedup();
        Self {
            size: comp.len(),
            sorted_array: comp,
        }
    }

    /// 圧縮後の番号を返す
    pub fn idx(&self, val: &T) -> Option<usize> {
        let idx = self.sorted_array.binary_search(&val);
        if let Ok(idx) = idx {
            Some(idx)
        } else {
            None
        }
    }

    /// 圧縮前の要素を返す
    pub fn val(&self, idx: usize) -> Option<&T> {
        if let Some(&val) = self.sorted_array.get(idx) {
            Some(val)
        } else {
            None
        }
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        T: usize,
        ts: [usize; N],
    }

    // 座標圧縮
    let mut times = vec![];
    for &t in &ts {
        times.push(t);
        times.push(t + T);
    }

    let comp = Compression::new(&times);
    debug!(&comp);

    // いもす法
    let mut imos = vec![0; comp.size];

    for &t in &ts {
        let begin = comp.idx(&t).unwrap();
        let end = comp.idx(&(t + T)).unwrap();
        imos[begin] += 1;
        imos[end] -= 1;
    }

    debug!(&imos);

    // 累積和を取る
    let mut ans = 0;
    let mut cnt = 1;

    for i in 1..comp.size {
        let diff = (*comp.val(i).unwrap()) - (*comp.val(i - 1).unwrap());

        if cnt > 0 {
            ans += diff;
        }

        cnt += imos[i];
    }

    println!("{}", ans);
}
