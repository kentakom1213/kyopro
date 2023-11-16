// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

use rustc_hash::FxHashMap;
use std::{collections::{BTreeSet, BTreeMap}, hash::Hash};
#[derive(Debug)]
pub struct MultiSet<T> {
    pub counter: FxHashMap<T, usize>,
    pub items: BTreeSet<(T, usize)>,
}
impl<T> MultiSet<T>
where
    T: Ord + Hash + Copy,
{
    /// MultiSetを初期化する
    pub fn new() -> Self {
        MultiSet {
            counter: FxHashMap::default(),
            items: BTreeSet::new(),
        }
    }
    /// 要素`x`を追加する
    pub fn insert(&mut self, x: T) {
        // カウンターに追加
        let cnt = self.counter.entry(x).or_insert(0);
        // setに追加
        self.items.insert((x, *cnt));
        // カウント
        *cnt += 1;
    }
    /// 要素`x`を削除する
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(v) = self.counter.get_mut(x) {
            // カウンターをデクリメント
            *v -= 1;
            // setから削除
            self.items.remove(&(*x, *v));
            return true;
        }
        false
    }
    /// 要素`x`が存在するか判定する
    pub fn contains(&self, x: &T) -> bool {
        self.counter.get(x).is_some_and(|cnt| *cnt > 0)
    }
    /// 先頭の要素を取得する
    pub fn first(&self) -> Option<&T> {
        self.items.first().map(|(ref x, _)| x)
    }
    /// 末尾の要素を取得する
    pub fn last(&self) -> Option<&T> {
        self.items.last().map(|(ref x, _)| x)
    }
    /// `x`の個数をカウントする
    pub fn count(&self, x: &T) -> usize {
        match self.counter.get(x) {
            Some(&v) => v,
            None => 0,
        }
    }
    /// 要素をすべて削除する
    pub fn clear(&mut self) {
        self.counter.clear();
        self.items.clear();
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<T> MultiSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().map(|(ref x, _)| x)
    }
}
impl<T: Ord + Hash + Copy> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut multiset = MultiSet::new();
        for x in iter {
            multiset.insert(x);
        }
        multiset
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut cards = BTreeMap::new();
    let mut boxes: BTreeMap<usize, MultiSet<usize>> = BTreeMap::new();

    // クエリの処理
    for _ in 0..Q {
        input!{t: usize}

        match t {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                boxes.entry(j).or_insert_with(|| MultiSet::new()).insert(i);
                cards.entry(i).or_insert_with(|| BTreeSet::new()).insert(j);
            }
            2 => {
                input! {
                    i: usize,
                }
                println!("{}", boxes[&i].iter().join(" "));
            }
            3 => {
                input! {
                    i: usize,
                }
                println!("{}", cards[&i].iter().join(" "));
            }
            _ => ()
        }
    }
}
