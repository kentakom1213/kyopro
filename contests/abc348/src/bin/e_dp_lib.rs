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

    let mut dp = RerootingDP::<Add, _, _>::new(N, |&v, i| v, |&v, i| v + C[i]);

    for (i, &(u, v)) in AB.iter().enumerate() {
        dp.add_edge(u, v, i, i);
    }

    let mut vdp = vec![0; N];
    let mut edp = vec![vec![]; N];
    dp.aggregate(INF, 0, &mut vdp, &mut edp);

    debug!(vdp);
    debug2D!(edp);

    let mut agg = vec![0; N];
    dp.reroot(INF, 0, &vdp, &edp, &mut agg);

    debug!(agg);
}

mod rerooting {
    use std::marker::PhantomData;

    use crate::{debug, monoid::Monoid};

    #[derive(Debug, Clone)]
    struct Edge {
        // 終点
        to: usize,
        // 辺の番号
        idx: usize,
        // 逆辺の番号
        ridx: usize,
    }

    pub struct RerootingDP<M: Monoid, F, G> {
        /// 頂点数
        N: usize,
        /// 値に辺`i`を作用させる関数
        apply_edge: F,
        /// 値に頂点`i`を作用させる関数
        apply_vertex: G,
        /// グラフ
        G: Vec<Vec<Edge>>,
        /// 幽霊型
        phantom: PhantomData<M>,
    }

    impl<M, F, G> RerootingDP<M, F, G>
    where
        M: Monoid,
        F: Fn(&M::Val, usize) -> M::Val,
        G: Fn(&M::Val, usize) -> M::Val,
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
        pub fn new(N: usize, apply_edge: F, apply_vertex: G) -> Self {
            Self {
                N,
                apply_edge,
                apply_vertex,
                G: vec![vec![]; N],
                phantom: PhantomData,
            }
        }

        /// 辺の追加
        /// - `u`: 始点
        /// - `v`: 終点
        pub fn add_edge(&mut self, u: usize, v: usize, idx: usize, ridx: usize) {
            self.G[u].push(Edge { to: v, idx, ridx });
            self.G[v].push(Edge { to: u, idx, ridx });
        }

        /// 頂点uに値を集約する
        /// - `dp`: 頂点uに集約された値
        /// - `edp[u][i]`: 頂点uのi番目の辺に集約された値
        pub fn aggregate(
            &self,
            p: usize,
            u: usize,
            vdp: &mut Vec<M::Val>,
            edp: &mut Vec<Vec<M::Val>>,
        ) {
            // 辺の集約値
            let mut tmp = M::E;

            for e in &self.G[u] {
                if e.to == p {
                    continue;
                }
                self.aggregate(u, e.to, vdp, edp);
                // 頂点e.toの値に辺eを作用
                let val = (self.apply_edge)(&vdp[e.to], e.idx);
                edp[u].push(val.clone());
                // 集約
                tmp = M::op(&tmp, &val);
            }
            // 自分の頂点を作用
            vdp[u] = (self.apply_vertex)(&tmp, u);
        }

        /// rerooting処理し，各頂点の値を求める
        /// - `u`: 現在見ている頂点
        /// - `edp[u][i]`: 頂点uのi番目の辺に集約された値
        /// - `dp[u]`: 頂点uを親としたときに集約される値
        pub fn reroot(
            &self,
            p: usize,
            u: usize,
            vdp: &Vec<M::Val>,
            edp: &Vec<Vec<M::Val>>,
            dp: &mut Vec<M::Val>,
        ) {
            // 両側からの累積和
            let acc = Self::accum(&edp[u]);
            // 頂点の値
            dp[u] = (self.apply_vertex)(&acc(None), u);

            for (i, e) in self.G[u].iter().enumerate() {
                let nxt_val = M::op(&vdp[u], &acc(Some(i)));
                debug!(i, e, nxt_val);
                // 辺の値を適用
            }
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
