#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

use crate::{monoid::examples::Add, rerooting::RerootingDP};

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        C: [isize; N]
    }

    let mut dp = RerootingDP::<Add, _>::new(N, |&v, i| v + C[i]);

    for (i, &(u, v)) in AB.iter().enumerate() {
        dp.add_edge(u, v, 0);
    }
}

mod rerooting {
    use crate::monoid::Monoid;

    #[derive(Debug, Clone)]
    pub struct Edge<M>
    where
        M: Monoid,
        M::Val: Clone,
    {
        /// 始点
        from: usize,
        /// 終点
        to: usize,
        /// 重み
        w: M::Val,
    }

    impl<M: Monoid> Default for Edge<M> {
        fn default() -> Self {
            Self {
                from: 0,
                to: 0,
                w: M::E,
            }
        }
    }

    impl<M> Edge<M>
    where
        M: Monoid,
        M::Val: Clone,
    {
        fn duplicate(&self) -> Self {
            Self {
                from: self.from,
                to: self.to,
                w: self.w.clone(),
            }
        }
    }

    pub struct RerootingDP<M: Monoid, F> {
        /// 頂点数
        N: usize,
        /// 値に辺`i`を作用させる関数
        apply_edge: F,
        /// グラフ
        G: Vec<Vec<Edge<M>>>,
    }

    impl<M, F> RerootingDP<M, F>
    where
        M: Monoid,
        F: Fn(&M::Val, usize) -> M::Val,
    {
        /// `i`番目の値を除く累積値を求められる累積和
        pub fn accum(array: &Vec<M::Val>) -> impl Fn(Option<usize>) -> M::Val {
            let n = array.len();
            // 順方向の累積和
            let mut S = vec![M::E; n + 1];
            for i in 0..n {
                S[i + 1] = M::op(&S[i], &array[i]);
            }
            // 逆方向の累積和
            let mut revS = vec![M::E; n + 1];
            for i in (1..=n).rev() {
                revS[i - 1] = M::op(&array[i - 1], &revS[i]);
            }
            // Some(i) => iを除く累積値
            // None => すべての集約値
            move |i: Option<usize>| {
                if let Some(i) = i {
                    M::op(&S[i], &revS[i + 1])
                } else {
                    revS[0].clone()
                }
            }
        }

        /// 全方位木dpの初期化
        /// - `N`: グラフの頂点数
        pub fn new(N: usize, apply_edge: F) -> Self {
            Self {
                N,
                apply_edge,
                G: (0..N).map(|_| Vec::new()).collect(),
            }
        }

        /// 辺の追加
        /// - `u`: 始点
        /// - `v`: 終点
        /// - `weight`: 重み
        pub fn add_edge(&mut self, u: usize, v: usize, weight: M::Val) {
            self.G[u].push(Edge {
                from: u,
                to: v,
                w: weight.clone(),
            });
            self.G[v].push(Edge {
                from: v,
                to: u,
                w: weight,
            });
        }

        /// 辺の追加（重みが異なる場合）
        /// - `u`: 始点
        /// - `v`: 終点
        /// - `uv_weight`: `u -> v` の重み
        /// - `vu_weight`: `v -> u` の重み
        pub fn add_edge2(&mut self, u: usize, v: usize, uv_weight: M::Val, vu_weight: M::Val) {
            self.G[u].push(Edge {
                from: u,
                to: v,
                w: uv_weight,
            });
            self.G[v].push(Edge {
                from: v,
                to: u,
                w: vu_weight,
            });
        }

        /// 全方位木DPを行う
        ///
        /// ### return
        /// - 各頂点についてその頂点を根として集約したときの値
        pub fn aggregate(&self) -> Vec<M::Val> {
            // ===== 木の整備 =====
            // 頂点iの親
            let mut parent: Vec<Edge<M>> = (0..self.N).map(|_| Edge::default()).collect();
            // BFS順の頂点列
            let mut bfs_ordering = vec![0];

            for i in 0..self.N {
                let u = bfs_ordering[i];

                for e in &self.G[u] {
                    if parent[u].to == e.to {
                        parent[u] = e.duplicate();
                    }
                }
            }

            todo!()
        }
    }
}

mod monoid {
    #![allow(dead_code)]
    //! モノイド
    use std::fmt::Debug;
    /// モノイド
    pub trait Monoid {
        /// 元の型
        type Val: Debug + Clone + PartialEq;
        /// 単位元
        const E: Self::Val;
        /// 演算
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
    }
    /// 各種モノイド
    pub mod examples {
        use super::Monoid;
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            const E: Self::Val = 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = isize;
            const E: Self::Val = (1 << 31) - 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = isize;
            const E: Self::Val = -((1 << 31) - 1);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                gcd(*left, *right)
            }
        }
        pub fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        /// アフィン変換（浮動小数点数）
        struct Affine;
        impl Monoid for Affine {
            type Val = (f64, f64);
            const E: Self::Val = (1.0, 0.0);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                let &(a1, b1) = left;
                let &(a2, b2) = right;
                (a2 * a1, a2 * b1 + b2)
            }
        }
    }
}

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
