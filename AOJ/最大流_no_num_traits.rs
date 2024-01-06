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
    let f = G.max_flow(0, V - 1);

    println!("{}", f);
}

pub mod ford_fulkerson {
    /// 辺
    #[derive(Debug, Clone)]
    pub struct Edge {
        /// 逆向きの辺の番号
        rev: usize,
        /// 辺の始点(from)
        from: usize,
        /// 辺の終点(to)
        to: usize,
        /// 残っている容量（capacity）
        cap: usize,
        /// 元の容量
        original_cap: usize,
    }

    impl Edge {
        pub fn new(rev: usize, from: usize, to: usize, cap: usize) -> Self {
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
    pub struct Graph(Vec<Vec<Edge>>);

    impl Graph {
        /// 空グラフを作成
        pub fn new(V: usize) -> Self {
            Self(vec![vec![]; V])
        }

        /// 逆向きの辺を返す
        pub fn rev_edge(&mut self, edge: &Edge) -> &mut Edge {
            &mut self.0[edge.to][edge.rev]
        }

        /// グラフに辺を追加する
        pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
            let from_size = self.0[from].len();
            let to_size = self.0[to].len();
            self.0[from].push(Edge::new(to_size, from, to, cap));
            self.0[to].push(Edge::new(from_size, to, from, 0));
        }

        /// 増加可能経路を見つけて、増加分のフローを返す
        fn dfs(graph: &mut Graph, used: &mut Vec<bool>, v: usize, t: usize, f: usize) -> usize {
            if v == t {
                return f;
            }
            used[v] = true;
            for i in 0..graph.0[v].len() {
                let Edge {
                    rev,
                    from,
                    to,
                    cap,
                    original_cap,
                } = graph.0[v][i];
                if !used[to] && cap > 0 {
                    let d = Self::dfs(graph, used, to, t, f.min(cap));
                    if d > 0 {
                        graph.0[v][i].cap -= d;
                        graph.rev_edge(&graph.0[v][i].clone()).cap += d;
                        return d;
                    }
                }
            }
            0
        }

        /// 最大流を求める
        pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
            let mut flow = 0;
            let mut used;
            loop {
                used = vec![false; self.0.len()];
                let f = Graph::dfs(self, &mut used, s, t, std::usize::MAX);
                if f == 0 {
                    return flow;
                } else {
                    flow += f;
                }
            }
        }
    }
}
