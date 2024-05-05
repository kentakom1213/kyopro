// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use std::collections::BTreeMap;
#[derive(Debug)]
pub struct MultiSet<T> {
    pub map: BTreeMap<T, usize>,
    len: usize,
}
impl<T> MultiSet<T>
where
    T: Ord,
{
    /// MultiSetを初期化する
    pub fn new() -> Self {
        MultiSet {
            map: BTreeMap::new(),
            len: 0,
        }
    }
    /// 要素`x`を追加する
    pub fn insert(&mut self, x: T) {
        *self.map.entry(x).or_insert(0) += 1;
        self.len += 1;
    }
    /// 要素`x`を削除する
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(v) = self.map.get_mut(x) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(x);
            }
            self.len -= 1;
            return true;
        }
        false
    }
    /// 要素`x`が存在するか判定する
    pub fn contains(&self, x: &T) -> bool {
        self.map.contains_key(x)
    }
    /// 先頭の要素を取得する
    pub fn first(&self) -> Option<&T> {
        self.map.keys().next()
    }
    /// 末尾の要素を取得する
    pub fn last(&self) -> Option<&T> {
        self.map.keys().last()
    }
    /// `x`の個数をカウントする
    pub fn count(&self, x: &T) -> usize {
        match self.map.get(x) {
            Some(&v) => v,
            None => 0,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        Q: usize,
    }

    let mut A = MultiSet::new();

    'out: for i in 0..Q {
        input!{ t: usize }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                A.insert(x);
            }
            2 => {
                input! {
                    x: usize,
                    k: usize,
                }
                let mut cur = 0;
                for (&v, &cnt) in A.map.range(..=x).rev() {
                    cur += cnt;
                    if cur >= k {
                        println!("{}", v);
                        continue 'out;
                    }
                }
                println!("-1");
            }
            3 => {
                input! {
                    x: usize,
                    k: usize,
                }
                let mut cur = 0;
                for (&v, &cnt) in A.map.range(x..) {
                    cur += cnt;
                    if cur >= k {
                        println!("{}", v);
                        continue 'out;
                    }
                }
                println!("-1");
            }
            _ => ()
        }
    }
}
