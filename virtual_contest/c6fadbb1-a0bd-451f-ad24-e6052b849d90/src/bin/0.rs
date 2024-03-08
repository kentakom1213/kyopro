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
use tree_dp::Monoid;

use crate::tree_dp::TreeDP;

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
        UV: [(Usize1, Usize1); N - 1]
    }

    let mut dp = TreeDP::<SubTree>::new(N);

    for &(u, v) in &UV {
        dp.add_edge(u, v);
    }

    dp.aggregate(0);

    debug!(dp.dp);

    // 最小値
    let mut sum = 0;
    let mut max = 0;
    for &v in &dp.G[0] {
        sum += dp.dp[v];
        max = max.max(dp.dp[v]);
    }

    let ans = sum - max + 1;
    println!("{ans}");
}

struct SubTree;
impl Monoid for SubTree {
    type Val = usize;
    const E: Self::Val = 1;
    fn apply(val: &Self::Val) -> Self::Val {
        *val
    }
    fn op(u: &Self::Val, v: &Self::Val) -> Self::Val {
        u + v
    }
}

mod tree_dp {
    //! 木DP
    type Graph = Vec<Vec<usize>>;
    /// # Monoid
    pub trait Monoid {
        type Val: Clone + PartialEq;
        const E: Self::Val;
        fn op(u: &Self::Val, v: &Self::Val) -> Self::Val;
        fn apply(val: &Self::Val) -> Self::Val;
    }
    /// # 木DP
    pub struct TreeDP<T: Monoid> {
        pub N: usize,
        pub G: Graph,
        pub dp: Vec<T::Val>,
    }
    impl<T: Monoid> TreeDP<T> {
        /// 頂点数`N`でグラフを初期化する
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
}
