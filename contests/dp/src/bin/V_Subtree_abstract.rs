//               V - Subtree               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_v
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::convert::TryInto;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## Fp
/// 有限体の実装
pub trait Fp {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Fp for usize {
    fn val(&self) -> usize {
        self % unsafe { MOD }
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (unsafe { MOD } - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(unsafe { MOD } - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

type Graph = Vec<Vec<usize>>;

/// # Monoid
pub trait Monoid {
    type Val: Clone + PartialEq;
    const E: Self::Val;
    fn op(u: &Self::Val, v: &Self::Val) -> Self::Val;
    fn apply(val: &Self::Val) -> Self::Val;
}

/// # 木DP
struct TreeDP<T: Monoid> {
    pub N: usize,
    pub G: Graph,
    dp: Vec<T::Val>,
}

impl<T: Monoid> TreeDP<T> {    
    pub fn new(N: usize) -> Self {
        Self {
            N,
            G: vec![vec![]; N],
            dp: vec![T::E; N],
        }
    }

    /// 辺`u`-`v`を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.G[u].push(v);
        self.G[v].push(u);
    }

    /// 頂点`start`に値を集約する
    pub fn aggregate(&mut self, start: usize) -> T::Val {
        let NEG1 = 1_usize.wrapping_neg();
        Self::dfs(
            NEG1,
            start,
            &self.G,
            &mut self.dp,
        );

        self.dp[start].clone()
    }

    fn dfs(p: usize, u: usize, G: &Graph, dp: &mut Vec<T::Val>) {
        // 葉であるときの処理
        if G[u].len() == 1 && G[u][0] == p {
            dp[u] = T::E;
            return;
        }

        // 子要素を集約する
        let mut acc = T::E; // 子要素の累積
        for &v in &G[u] {
            if v == p {
                continue;
            }
            Self::dfs(u, v, G, dp);
            acc = T::op(
                &acc,
                &dp[v]
            );
        }
        dp[u] = T::apply(&acc);
    }
}

static mut MOD: usize = 1;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // MODを設定
    unsafe {
        MOD = M;
    }

    struct DP;
    impl Monoid for DP {
        type Val = (usize, usize); // (白, 黒, unsafe { MOD })
        const E: Self::Val = (1, 1);
        fn op(u: &Self::Val, v: &Self::Val) -> Self::Val {
            let u0 = u.0.mmul(v.0);
            let u1 = u.1.mmul(v.0.madd(v.1));
            (u0, u1)
        }
        fn apply(val: &Self::Val) -> Self::Val {
            val.clone()
        }
    }

    // 木DP
    let mut tree = TreeDP::<DP>::new(N);

    for &(u, v) in &edges {
        tree.add_edge(u, v);
    }

    for i in 0..N {
        let res = tree.aggregate(i);
        println!("{}", res.1);
    }
}
