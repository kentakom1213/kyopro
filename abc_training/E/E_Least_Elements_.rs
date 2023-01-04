//           E - Least Elements            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc281/tasks/abc281_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::Bound::{Included, Excluded, Unbounded};

/// # MultiSet
/// 多重集合
#[derive(Debug, Clone)]
struct MultiSet<T> {
    counter: BTreeMap<T, usize>,
    multiset: BTreeSet<(T, usize)>,
}

impl<T> MultiSet<T>
where T: Ord + Copy
{
    fn new() -> Self {
        MultiSet {
            counter: BTreeMap::new(),
            multiset: BTreeSet::new(),
        }
    }

    fn insert(&mut self, key: T) {
        // 現在の個数をカウント
        let cnt_key = self.counter.entry(key).or_insert(0);
        // 要素を挿入
        self.multiset.insert((key, *cnt_key));
        // 個数をインクリメント
        *cnt_key += 1;
    }

    fn remove(&mut self, key: T) -> bool {
        match self.counter.get_mut(&key) {
            Some(cnt_key) => {
                if *cnt_key == 0 {
                    false
                } else {
                    *cnt_key -= 1;
                    self.multiset.remove(&(key, *cnt_key));
                    true
                }
            },
            None => false,
        }
    }

    fn is_contain(&self, key: T) -> bool {
        match self.counter.get(&key) {
            Some(&cnt_key) => {
                if cnt_key == 0 {
                    false
                } else {
                    true
                }
            },
            None => false,
        }
    }

    fn is_empty(&self) -> bool {
        self.multiset.is_empty()
    }

    fn len(&self) -> usize {
        self.multiset.len()
    }

    fn first(&self) -> Option<T> {
        match self.multiset.iter().next() {
            Some(&(key, _)) => Some(key),
            None => None,
        }
    }

    fn last(&self) -> Option<T> {
        match self.multiset.iter().next_back() {
            Some(&(key, _)) => Some(key),
            None => None,
        }
    }

    /// x以上の値を探索する
    fn lower_bound(&self, x: T) -> T {
        let mut greater_equal = self.multiset.range(
            (Included((x, 0)), Unbounded)
        );

        match greater_equal.next() {
            Some(&(key, _)) => key,
            None => panic!("No applicable value in this multiset."),
        }
    }

    /// xより大きい値を探索する
    fn upper_bound(&self, x: T) -> T {
        let lb_x = self.lower_bound(x);
        let cnt_key = self.counter[&lb_x];
        let mut greater_equal = self.multiset.range(
            (Excluded((x, cnt_key)), Unbounded)
        );

        match greater_equal.next() {
            Some(&(key, _)) => key,
            None => panic!("No applicable value in this multiset."),
        }
    }

    fn count(&self, key: T) -> usize {
        match self.counter.get(&key) {
            Some(&cnt_key) => cnt_key,
            None => 0,
        }
    }
}

impl<T> IntoIterator for MultiSet<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.multiset
            .into_iter()
    }
}

// input macro
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 尺取り法
/// - MultiSet 2本で区間を管理する
fn main() {
    let (N, M, K) = get!(usize, usize, usize);
    let A = get!(usize;;);

    // 多重集合2個で管理
    let mut L = MultiSet::new();  // 小さい方からK個
    let mut R = MultiSet::new();  // それ以外
    let mut sum = 0;  // 小さい方からK個の和

    // 先頭K個を抽出
    let mut head_M = A[..M].to_vec();
    head_M.sort();

    // 初期化
    for (i, v) in head_M.into_iter().enumerate() {
        if i < K {
            L.insert(v);
            sum += v;
        } else {
            R.insert(v);
        }
    }

    print!("{} ", sum);

    // 区間の処理
    for i in 0..N-M {
        // A[i] を区間から削除
        if let Some(v) = L.last() {
            if A[i] <= v {
                L.remove(A[i]);
                sum -= A[i];
            } else {
                R.remove(A[i]);
            }
        } else {
            R.remove(A[i]);
        }

        // A[i+M] を区間に追加
        if let Some(v) = L.last() {
            if A[i+M] <= v {
                L.insert(A[i+M]);
                sum += A[i+M];
            } else {
                R.insert(A[i+M]);
            }
        } else {
            R.insert(A[i+M]);
        }

        if L.len() < K {
            // min(R) -> L
            let minR = R.first().unwrap();
            R.remove(minR);
            L.insert(minR);
            sum += minR;
        } else if L.len() > K {
            // max(L) -> R
            let maxL = L.last().unwrap();
            L.remove(maxL);
            R.insert(maxL);
            sum -= maxL;
        }

        print!("{} ", sum);
    }

    println!();
}
