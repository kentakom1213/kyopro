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

/// SegmentTree (Monoid)
struct SegmentTree<F, T> {
    offset: usize,
    data: Vec<T>,
    op: F,
    e: T,
}

impl<F, T> SegmentTree<F, T>
where
    F: Fn(T, T) -> T,
    T: Copy + Eq + std::fmt::Debug,
{
    /// ## new
    /// セグメント木を初期化する
    fn new(n: usize, e: T, op: F) -> Self {
        let len = n.next_power_of_two();

        Self {
            offset: len,
            data: vec![e; len << 1],
            op: op,
            e: e,
        }
    }

    /// ## from
    /// 配列からセグメント木を生成する
    fn from(arr: &[T], e: T, op: F) -> Self {
        let mut seg = Self::new(arr.len(), e, op);
        for (i, &v) in arr.iter().enumerate() {
            seg.data[seg.offset + i] = v;
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = (seg.op)(seg.data[lch], seg.data[lch + 1]);
        }
        seg
    }

    /// ## update
    /// 要素`index`を`value`に上書きする
    /// （`index`：0-indexed）
    fn update(&mut self, index: usize, value: T) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = (self.op)(self.data[lch], self.data[lch + 1]);
        }
    }

    /// ## get_point
    /// 一点取得する
    fn get_point(&self, index: usize) -> T {
        let i = self.offset + index;
        self.data[i]
    }

    /// ## get_range
    /// 区間`[l, r)`を取得する
    fn get_range(&self, left: usize, right: usize) -> T {
        let mut l = self.offset + left;
        let mut r = self.offset + right;
        let (mut res_l, mut res_r) = (self.e, self.e);

        while l < r {
            if l & 1 == 1 {
                res_l = (self.op)(res_l, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(self.data[r], res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(res_l, res_r)
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
const INF: isize = 1001001001001001001;

/// ## 方針
/// - セグ木を使う解法
fn main() {
    let (N, M, K) = get!(usize, usize, usize);
    let A = get!(isize;;);

    // M == Kのとき
    if M == K {
        let sum = SegmentTree::from(&A, 0, |a, b| a + b);
        for i in 0..N-M+1 {
            print!("{} ", sum.get_range(i, i+K));
        }
        println!();
        return;
    }

    let mut sum0 = SegmentTree::new(N, 0, |a, b| a + b);
    let mut sum1 = SegmentTree::new(N, 0, |a, b| a + b);
    let mut used = SegmentTree::new(
        N,
        (-INF, 0_usize),
        |a, b| a.max(b)
    );
    let mut unused = SegmentTree::new(
        N,
        (INF, 0_usize),
        |a, b| a.min(b)
    );

    // 初期化
    for i in 0..N {
        used.update(i, (-INF, i));
        unused.update(i, (INF, i));
    }

    // 区間の処理
    for i in 0..N {
        // push(i)
        sum0.update(i, 1);
        sum1.update(i, A[i]);
        used.update(i, (A[i], i));

        if sum0.get_range(0, N) > K {
            // Lの最大値を使わない
            let pop = used.get_range(0, N).1;  // Lの最大値のindex
            sum0.update(pop, 0);
            sum1.update(pop, 0);
            used.update(pop, (-INF, pop));
            unused.update(pop, (A[pop], pop));
        }

        if i >= M - 1 {
            print!("{} ", sum1.get_range(0, N));  // 答えの出力

            let j = i - (M - 1);
            if sum0.get_point(j) > 0 {
                // pop(j)
                sum0.update(j, 0);
                sum1.update(j, 0);
                used.update(j, (-INF, j));

                // Rの最小値を取り出し
                let push = unused.get_range(0, N).1;
                sum0.update(push, 1);
                sum1.update(push, A[push]);
                used.update(push, (A[push], push));
                unused.update(push, (INF, push));
            } else {
                // pop(j)
                unused.update(j, (INF, j));
            }
        }
    }
    println!();
}
