#![allow(non_snake_case)]

use crate::cp_library_rs::{ford_fulkerson::FordFulkerson, get};

fn main() {
    let (X, Y, E) = get!(usize, usize, usize);
    let edges = get!(usize, usize; E);

    let mut ff = FordFulkerson::new(X + Y + 2);

    let S = X + Y;
    let T = S + 1;

    for &(u, v) in &edges {
        ff.add_edge(u, v + X, 1);
    }

    for u in 0..X {
        ff.add_edge(S, u, 1);
    }

    for v in 0..Y {
        ff.add_edge(v + X, T, 1);
    }

    // フローを流す
    let res = ff.max_flow(S, T);

    println!("{}", res);
}

mod num_traits {
    use std::ops::{Add, Sub};

    pub trait PrimInt
    where
        Self: Clone + Copy + Ord + Add<Output = Self> + Sub<Output = Self>,
    {
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
        fn max_value() -> Self;
    }

    impl PrimInt for usize {
        fn zero() -> Self {
            0
        }
        fn is_zero(&self) -> bool {
            *self == 0
        }
        fn max_value() -> Self {
            std::usize::MAX
        }
    }
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod get {
        //! 入力用マクロ
        //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
        /// 入力用マクロ
        #[macro_export]
        macro_rules! get {
            (parse, $val:expr, usize1) => {(get!(parse, $val, usize) - 1)};
            (parse, $val:expr, chars) => {get!(parse, $val, String).chars().collect::<Vec<_>>()};
            (parse, $val:expr, $t:ty) => {$val.parse::<$t>().unwrap()};
            ($p:tt) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    get!(parse, line.trim(), $p)
            }};
            ($($p:tt),*) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    let mut iter = line.split_whitespace();
                    ( $(get!(parse, iter.next().unwrap(), $p),)* )
            }};
            ($t:tt ; $n:expr) => {(0..$n).map(|_| get!($t)).collect::<Vec<_>>()};
            ($($t:tt),* ; $n:expr) => {(0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()};
            ($t:tt ;;) => {{
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).ok();
                    line.split_whitespace().map(|v| get!(parse, v, $t)).collect::<Vec<_>>()
            }};
            ($t:tt ;; $n:expr) => {(0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()};
        }
    }
    pub mod ford_fulkerson {
        //! Ford-Fulkerson法
        use crate::num_traits::PrimInt;
        use std::collections::HashMap;
        #[derive(Debug, Clone)]
        pub struct Edge<T> {
            /// 逆向きの辺の番号
            rev: usize,
            from: usize,
            to: usize,
            cap: T,
            original_cap: T,
        }
        impl<T: Clone> Edge<T> {
            pub fn new(rev: usize, from: usize, to: usize, cap: T) -> Self {
                Self {
                    rev,
                    from,
                    to,
                    cap: cap.clone(),
                    original_cap: cap,
                }
            }
        }
        type Graph<T> = Vec<Vec<Edge<T>>>;
        /// FordFulkerson法を実行する
        #[derive(Debug)]
        pub struct FordFulkerson<T> {
            N: usize,
            /// 残余ネットワーク
            pub G: Graph<T>,
            /// 辺の番号
            edge_id: HashMap<(usize, usize), usize>,
        }
        impl<T: PrimInt> FordFulkerson<T> {
            pub fn new(n: usize) -> Self {
                Self {
                    N: n,
                    G: vec![vec![]; n],
                    edge_id: HashMap::default(),
                }
            }
            /// 有向辺を追加する
            pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
                let rev = self.G[to].len();
                self.G[from].push(Edge::new(rev, from, to, cap));
                // 逆向き辺を追加
                let rrev = self.G[from].len() - 1;
                self.G[to].push(Edge::new(rrev, to, from, T::zero()));
                // 辺のidを追加
                self.edge_id.insert((from, to), rrev);
            }
            /// uからtへのフロー追加路を見つける
            fn dfs(u: usize, t: usize, f: T, G: &mut Graph<T>, visit: &mut [bool]) -> T {
                if u == t {
                    return f;
                }
                visit[u] = true;
                for i in 0..G[u].len() {
                    let v = G[u][i].to;
                    let cap = G[u][i].cap;
                    if visit[v] || cap <= T::zero() {
                        continue;
                    }
                    let d = Self::dfs(v, t, f.min(cap), G, visit);
                    if d > T::zero() {
                        // 前向き辺の更新
                        G[u][i].cap = cap - d;
                        // 後ろ向き辺の更新
                        let r = G[u][i].rev;
                        G[v][r].cap = G[v][r].cap + d;
                        return d;
                    }
                }
                T::zero()
            }
            /// 最大流を求める
            pub fn max_flow(&mut self, s: usize, t: usize) -> T {
                let mut flow = T::zero();
                let mut visit = vec![false; self.N];
                loop {
                    visit = vec![false; visit.len()];
                    // フロー追加路を見つける
                    let f = Self::dfs(s, t, T::max_value(), &mut self.G, &mut visit);
                    if f.is_zero() {
                        break;
                    }
                    flow = flow + f;
                }
                flow
            }
            /// 現在の状態での辺(u,v)の流量を求める
            pub fn get_flow(&self, u: usize, v: usize) -> T {
                let i = self.edge_id[&(u, v)];
                let cap = self.G[u][i].cap;
                let original = self.G[u][i].original_cap;
                original - cap
            }
        }
    }
}
