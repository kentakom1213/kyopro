//                   最大流
// ----------------------------------------
// 問題
// https://judge.from-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A&lang=jp
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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

use ford_fulkerson::*;

fn main() {
    let (V, E) = get!(usize, usize);
    let mut G = (0..E).fold(Graph::new(V), |mut g, _| {
        let (from, to, cap) = get!(usize, usize, usize);
        g.add_edge(from, to, cap);
        g
    });
    let f = max_flow(&mut G, 0, V - 1);

    println!("{}", f);
}

pub mod ford_fulkerson {
    use num_traits::{NumAssignOps, PrimInt};

    /// 辺
    #[derive(Debug, Clone)]
    pub struct Edge<T: PrimInt> {
        /// 逆向きの辺の番号
        rev: usize,
        /// 辺の始点(from)
        from: usize,
        /// 辺の終点(to)
        to: usize,
        /// 残っている容量（capacity）
        cap: T,
        /// 元の容量
        original_cap: T,
    }

    impl<T: PrimInt> Edge<T> {
        pub fn new(rev: usize, from: usize, to: usize, cap: T) -> Self {
            Self {
                rev,
                from,
                to,
                cap,
                original_cap: cap,
            }
        }
    }

    /// 残余ネットワーク
    pub struct Graph<T: PrimInt> {
        graph: Vec<Vec<Edge<T>>>,
    }

    impl<T: PrimInt> Graph<T> {
        /// 空グラフを作成
        pub fn new(V: usize) -> Self {
            Self {
                graph: vec![vec![]; V],
            }
        }

        /// 逆向きの辺を返す
        pub fn rev_edge(&mut self, edge: &Edge<T>) -> &mut Edge<T> {
            &mut self.graph[edge.to][edge.rev]
        }

        /// グラフに辺を追加する
        pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
            let from_size = self.graph[from].len();
            let to_size = self.graph[to].len();
            self.graph[from].push(Edge::new(to_size, from, to, cap));
            self.graph[to].push(Edge::new(from_size, to, from, T::zero()));
        }
    }

    /// 増加可能経路を見つけて、増加分のフローを返す
    fn dfs<T: PrimInt + NumAssignOps>(
        graph: &mut Graph<T>,
        used: &mut Vec<bool>,
        v: usize,
        t: usize,
        f: T,
    ) -> T {
        debug!(v, t);
        if v == t {
            return f;
        }
        used[v] = true;
        for i in 0..graph.graph[v].len() {
            let Edge {
                rev,
                from,
                to,
                cap,
                original_cap,
            } = graph.graph[v][i];
            if !used[to] && cap > T::zero() {
                let d = dfs(graph, used, to, t, f.min(cap));
                if d > T::zero() {
                    graph.graph[v][i].cap -= d;
                    graph.rev_edge(&graph.graph[v][i].clone()).cap += d;
                    return d;
                }
            }
        }
        T::zero()
    }

    /// 最大流を求める
    pub fn max_flow<T: PrimInt + NumAssignOps>(graph: &mut Graph<T>, s: usize, t: usize) -> T {
        let mut flow = T::zero();
        let mut used = vec![false; graph.graph.len()];
        loop {
            used.fill(false);
            let f = dfs(graph, &mut used, s, t, T::max_value());
            if f == T::zero() {
                return flow;
            } else {
                flow += f;
            }
        }
    }
}
