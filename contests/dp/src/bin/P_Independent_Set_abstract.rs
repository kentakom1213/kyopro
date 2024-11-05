//           P - Independent Set
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_p
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
    pub G: Vec<Vec<usize>>,
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
        Self::dfs(NEG1, start, &self.G, &mut self.dp);

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
            acc = T::op(&acc, &dp[v]);
        }
        dp[u] = T::apply(&acc);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // モノイドを定義
    struct Coloring;
    impl Monoid for Coloring {
        type Val = (usize, usize);
        const E: Self::Val = (1, 1);
        fn op(u: &Self::Val, v: &Self::Val) -> Self::Val {
            let u0 = u.0 * (v.0 + v.1) % MOD1;
            let u1 = u.1 * v.0 % MOD1;
            (u0, u1)
        }
        fn apply(val: &Self::Val) -> Self::Val {
            val.clone()
        }
    }

    // 木DP
    let mut tree = TreeDP::<Coloring>::new(N);

    for &(u, v) in &edges {
        tree.add_edge(u, v);
    }

    let res = tree.aggregate(0);
    let ans = (res.0 + res.1) % MOD1;

    println!("{}", ans);
}
